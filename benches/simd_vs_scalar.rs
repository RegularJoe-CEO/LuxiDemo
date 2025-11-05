/*
SPDX-FileCopyrightText: 2025 Eric Waller
SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0
*/

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use erock::{interpreter, lexer, parser};

fn build_ast() -> (parser::Arena, usize) {
    let input = "sum = 3.14 + (x - 2) * 10";
    let tokens = lexer::tokenize(input);
    parser::parse(tokens).expect("parse failed")
}

fn make_xs(n: usize) -> Vec<f64> {
    (0..n).map(|i| (i as f64) * 0.1).collect()
}

fn bench_scalar(c: &mut Criterion) {
    let (arena, root_idx) = build_ast();
    let xs = make_xs(100_000);

    c.bench_function("scalar_100k", |b| {
        b.iter(|| {
            let mut vars: HashMap<String, f64> = HashMap::with_capacity(2);
            vars.insert("x".to_string(), 0.0);
            let mut acc = 0.0f64;
            for &x in &xs {
                if let Some(v) = vars.get_mut("x") { *v = x; }
                acc += interpreter::interpret(root_idx, &arena, &mut vars);
            }
            black_box(acc)
        })
    });
}

fn bench_simd(c: &mut Criterion) {
    let (arena, root_idx) = build_ast();
    let xs = make_xs(100_000);
    let vars: HashMap<String, f64> = HashMap::new();

    c.bench_function("simd_100k_f64x4", |b| {
        b.iter(|| {
            let out = interpreter::simd_eval_over_x(root_idx, &arena, &vars, &xs);
            black_box(out.len())
        })
    });
}

fn benches(c: &mut Criterion) {
    bench_scalar(c);
    bench_simd(c);
}

criterion_group!(name = erock_benches; config = Criterion::default(); targets = benches);
criterion_main!(erock_benches);
