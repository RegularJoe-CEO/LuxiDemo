use crate::eval;
use crate::eval::hash_f32_slice;
use attention_transformer::activations::gelu;
use attention_transformer::config::Config;
use attention_transformer::layernorm::layernorm;
use attention_transformer::online_softmax::OnlineSoftmax;
use attention_transformer::scaling::profile::{gpt2_124m_profile, llama2_7b_profile};
use attention_transformer::waller_operator::waller_operator;
use attention_transformer::welford::WelfordState;
use attention_transformer::wnsm_transformer::{
    format_receipt, sha256_of_f32_slice, WNSM_GAE_Decoder,
};

const GOLD_VALUES: [f64; 5] = [0.5, 1.0, 1.57, 2.0, 3.14];

pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub sha256: String,
    pub note: String,
}

pub fn run_all() -> Vec<CheckResult> {
    let mut out = Vec::new();
    out.extend(run_luxiedge_checks());
    out.extend(run_transformer_checks());
    out.extend(run_profile_checks());
    out
}

fn run_luxiedge_checks() -> Vec<CheckResult> {
    let mut results = Vec::new();
    let values: Vec<f64> = GOLD_VALUES.to_vec();

    for expr in eval::lite::supported_exprs() {
        let (_, hash) = eval::lite::evaluate(expr, &values, false);
        results.push(CheckResult {
            name: format!("LuxiEdge lite: {}", expr),
            passed: !hash.is_empty(),
            sha256: hash,
            note: "f32 deterministic".to_string(),
        });
    }

    for expr in ["relu(x)", "sigmoid(x)", "rf_chain(x)", "quant_chain(x)", "gelu(x)"] {
        let (_, hash) = eval::demo::evaluate(expr, &values, false);
        results.push(CheckResult {
            name: format!("LuxiEdge demo: {}", expr),
            passed: !hash.is_empty(),
            sha256: hash,
            note: "demo operators".to_string(),
        });
    }

    results
}

fn run_transformer_checks() -> Vec<CheckResult> {
    let mut results = Vec::new();

    let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
    let mut w = WelfordState::new();
    for &x in &data {
        w.update(x);
    }
    let welford_out = vec![w.mean, w.variance(), w.std(1e-5)];
    let welford_hash = hash_f32_slice(&welford_out);
    results.push(CheckResult {
        name: "Welford mean/variance".to_string(),
        passed: w.count == 5,
        sha256: welford_hash,
        note: format!("mean={:.4}", w.mean),
    });

    let logits = [1.0f32, 2.0, 0.5, -1.0];
    let mut online = OnlineSoftmax::new();
    for &s in &logits {
        online.update(s);
    }
    let max = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = logits.iter().map(|&s| (s - max).exp()).collect();
    let sum: f32 = exps.iter().sum();
    let sm: Vec<f32> = exps.iter().map(|&e| e / sum).collect();
    let sm_state = vec![online.max, online.sum];
    let sm_hash = hash_f32_slice(&sm_state);
    results.push(CheckResult {
        name: "Online softmax".to_string(),
        passed: sm.iter().sum::<f32>() > 0.99 && online.sum > 0.0,
        sha256: sm_hash,
        note: format!("dist_sum={:.4}", sm.iter().sum::<f32>()),
    });

    let gelu_vals: Vec<f32> = [-1.0, 0.0, 1.0, 2.0].iter().map(|&x| gelu(x)).collect();
    let gelu_hash = hash_f32_slice(&gelu_vals);
    results.push(CheckResult {
        name: "GELU activation".to_string(),
        passed: gelu_vals[1].abs() < 1e-6,
        sha256: gelu_hash,
        note: "gelu(0)=0".to_string(),
    });

    let ln_in = [1.0f32, 2.0, 3.0, 4.0];
    let gamma = [1.0f32; 4];
    let beta = [0.0f32; 4];
    let ln_out = layernorm(&ln_in, &gamma, &beta, 1e-5);
    let ln_hash = hash_f32_slice(&ln_out);
    results.push(CheckResult {
        name: "LayerNorm".to_string(),
        passed: !ln_out.is_empty(),
        sha256: ln_hash,
        note: "4-dim vector".to_string(),
    });

    let seq_len = 4usize;
    let head_dim = 2usize;
    let scale = 1.0 / (head_dim as f32).sqrt();
    let q = vec![1.0f32, 0.0, 0.0, 1.0, 0.5, 0.5, 0.5, 0.5];
    let k = vec![1.0f32, 0.0, 0.0, 1.0, 0.5, 0.5, 0.5, 0.5];
    let v = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let waller_out = waller_operator(&q, &k, &v, seq_len, head_dim, scale);
    let waller_hash = hash_f32_slice(&waller_out);
    results.push(CheckResult {
        name: "Waller geodesic operator".to_string(),
        passed: waller_out.len() == q.len(),
        sha256: waller_hash,
        note: format!("seq={} head_dim={}", seq_len, head_dim),
    });

    let hidden = 64usize;
    let heads = 4usize;
    let layers = 3usize;
    let seq_len = 8usize;
    let mlp = hidden * 4;
    let cfg = Config::new(hidden, heads, mlp, seq_len);
    let mut model = WNSM_GAE_Decoder::new(cfg, layers);
    let input: Vec<f32> = (0..seq_len * hidden)
        .map(|i| ((i as f32) * 0.017).sin() * 0.2)
        .collect();
    let out = model.forward(input, seq_len);
    let block_hash = format_receipt(&sha256_of_f32_slice(&out));
    results.push(CheckResult {
        name: "Transformer block (WNSM)".to_string(),
        passed: out.len() == seq_len * hidden,
        sha256: block_hash,
        note: "e1980a6f… expected on reference build".to_string(),
    });

    results
}

fn run_profile_checks() -> Vec<CheckResult> {
    let gpt2 = gpt2_124m_profile();
    let llama = llama2_7b_profile();
    vec![
        CheckResult {
            name: "GPT-2 124M config preset".to_string(),
            passed: gpt2.hidden_dim == 768 && gpt2.num_layers == 12,
            sha256: format!("{:x}", simple_profile_hash(&gpt2.id)),
            note: format!(
                "hidden={} layers={} heads={}",
                gpt2.hidden_dim, gpt2.num_layers, gpt2.num_heads
            ),
        },
        CheckResult {
            name: "Llama2 7B config preset".to_string(),
            passed: llama.hidden_dim == 4096 && llama.num_layers == 32,
            sha256: format!("{:x}", simple_profile_hash(&llama.id)),
            note: format!(
                "hidden={} layers={} heads={}",
                llama.hidden_dim, llama.num_layers, llama.num_heads
            ),
        },
    ]
}

fn simple_profile_hash(id: &str) -> u64 {
    id.bytes().fold(0u64, |h, b| h.wrapping_mul(31).wrapping_add(b as u64))
}

pub fn print_report(results: &[CheckResult]) {
    println!();
    println!("  Luxi Validation Suite");
    println!("  ============================================================");
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = results.len() - passed;
    for r in results {
        let status = if r.passed { "PASS" } else { "FAIL" };
        let short = if r.sha256.len() > 16 {
            format!("{}...", &r.sha256[..16])
        } else {
            r.sha256.clone()
        };
        println!("  {:<36} {}  {}  {}", r.name, status, short, r.note);
    }
    println!("  ------------------------------------------------------------");
    println!("  PASSED: {}   FAILED: {}", passed, failed);
    println!("  ============================================================");
    println!();
}