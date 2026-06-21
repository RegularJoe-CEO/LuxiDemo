// Legacy benchmark (from the original repo scaffold).
// It now exercises the real WNSM production path for the chosen shape.
//
// For real performance & energy (calcs per joule) measurement:
//   - Use WNSM_GAE_Decoder::forward_cuda when the cuda feature is enabled.
//   - Profile data movement (the dominant cost) + kernel time on actual hardware.
//   - The fused CUDA / future mega kernels are designed for the highest ops/joule
//     by minimizing HBM traffic for attention and WNSM payload relay.

use attention_transformer::config::Config;
use attention_transformer::wnsm_transformer::WNSM_GAE_Decoder;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_transformer_forward(c: &mut Criterion) {
    // Use the real production API for meaningful performance & energy numbers.
    let hidden = 64;
    let heads = 4;
    let layers = 2;
    let seq = 64;
    let mlp = hidden * 4;

    let cfg = Config::new(hidden, heads, mlp, seq);
    let mut model = WNSM_GAE_Decoder::new(cfg, layers);

    let input: Vec<f32> = (0..seq * hidden)
        .map(|i| (i as f32 * 0.01).sin() * 0.2)
        .collect();

    c.bench_function("WNSM_GAE_Decoder::forward (real production path)", |b| {
        b.iter(|| {
            let out = model.forward(black_box(input.clone()), seq);
            black_box(out)
        })
    });
}

criterion_group!(benches, bench_transformer_forward);
criterion_main!(benches);
