// Microbenchmark: standard matmul_bias (on-the-fly B packing per tile)
// vs matmul_bias_packed (pre-packed B, zero runtime repacking).
//
// Tests realistic GPT-2 projection shapes:
//   - c_attn: [768, 2304]
//   - c_proj: [768, 768]
//   - c_fc:   [768, 3072]
//   - c_mlp:  [3072, 768]
//
// Both M=1 (incremental generation, hot path) and M=64 (prompt processing).

use attention_transformer::linalg::{matmul_bias, matmul_bias_packed, pack_b_for_kernel};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn make_matrix(rows: usize, cols: usize) -> Vec<f32> {
    (0..rows * cols)
        .map(|i| ((i as f32) * 0.01).sin() * 0.2)
        .collect()
}

fn make_bias(cols: usize) -> Vec<f32> {
    (0..cols).map(|i| (i as f32) * 0.001).collect()
}

fn bench_shape(c: &mut Criterion, name: &str, m: usize, k: usize, n: usize) {
    let a = make_matrix(m, k);
    let b = make_matrix(k, n);
    let bias = make_bias(n);
    let packed_b = pack_b_for_kernel(&b, k, n);

    let std_name = format!("{name}_standard_m{m}_k{k}_n{n}");
    let packed_name = format!("{name}_packed_m{m}_k{k}_n{n}");

    c.bench_function(&std_name, |bench| {
        bench.iter(|| {
            let out = matmul_bias(black_box(&a), black_box(&b), black_box(&bias), m, k, n);
            black_box(out)
        })
    });

    c.bench_function(&packed_name, |bench| {
        bench.iter(|| {
            let out = matmul_bias_packed(
                black_box(&a),
                black_box(&packed_b),
                black_box(&bias),
                m,
                k,
                n,
            );
            black_box(out)
        })
    });
}

fn bench_all(c: &mut Criterion) {
    // GPT-2 c_proj shape [768, 768]
    bench_shape(c, "c_proj", 1, 768, 768);
    bench_shape(c, "c_proj", 64, 768, 768);

    // GPT-2 c_attn shape [768, 2304]
    bench_shape(c, "c_attn", 1, 768, 2304);
    bench_shape(c, "c_attn", 64, 768, 2304);

    // GPT-2 c_fc shape [768, 3072]
    bench_shape(c, "c_fc", 1, 768, 3072);
    bench_shape(c, "c_fc", 64, 768, 3072);

    // GPT-2 c_mlp shape [3072, 768]
    bench_shape(c, "c_mlp", 1, 3072, 768);
    bench_shape(c, "c_mlp", 64, 3072, 768);
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
