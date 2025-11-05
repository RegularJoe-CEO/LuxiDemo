use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

use erock::luxi_eval::{interpreter, lexer, parser};

fn parse_expr(expr: &str) -> parser::Arena {
    let tokens = lexer::tokenize(expr);
    parser::parse(tokens).expect("parse failed").0
}

fn same_sign(a: f64, b: f64) -> bool {
    (a.is_sign_positive() && b.is_sign_positive()) || (a.is_sign_negative() && b.is_sign_negative())
}

fn fallback_bisect<F>(
    eval_at: &F,
    guess: f64,
    step: f64,
    max_expand: usize,
    tol: f64,
    max_iter: usize,
) -> f64
where
    F: Fn(f64) -> f64,
{
    let base_step = step.abs().max(1e-6);
    let f0 = eval_at(guess);
    if !f0.is_finite() {
        return f64::NAN;
    }
    if f0.abs() <= tol {
        return guess;
    }

    let mut s = base_step;
    let mut lo = f64::NAN;
    let mut hi = f64::NAN;

    for _ in 0..=max_expand {
        let a = guess - s;
        let fa = eval_at(a);
        if fa.is_finite() && !same_sign(fa, f0) {
            lo = a.min(guess);
            hi = a.max(guess);
            break;
        }

        let b = guess + s;
        let fb = eval_at(b);
        if fb.is_finite() && !same_sign(fb, f0) {
            lo = guess.min(b);
            hi = guess.max(b);
            break;
        }

        s *= 2.0;
    }

    if !lo.is_finite() || !hi.is_finite() {
        return f64::NAN;
    }

    let mut left = lo;
    let mut right = hi;
    let mut flo = eval_at(left);

    for _ in 0..max_iter {
        let mid = 0.5 * (left + right);
        let fm = eval_at(mid);

        if !fm.is_finite() {
            return mid;
        }

        if (right - left).abs() <= tol || fm.abs() <= tol {
            return mid;
        }

        if same_sign(fm, flo) {
            left = mid;
            flo = fm;
        } else {
            right = mid;
        }
    }

    0.5 * (left + right)
}

fn newton_with_fallback(
    arena: &parser::Arena,
    base: &HashMap<String, f64>,
    guess: f64,
    tol: f64,
    max_iter: usize,
) -> f64 {
    let eval_at = |x: f64| interpreter::eval_with_var(arena, base, "x", x);
    let mut current = guess;
    let mut value = eval_at(current);

    if value.is_finite() && value.abs() <= tol {
        return current;
    }

    for _ in 0..max_iter {
        if !value.is_finite() {
            break;
        }
        let deriv = interpreter::derivative_with_var(arena, base, "x", current, 1e-6);
        if !deriv.is_finite() || deriv.abs() < 1e-12 {
            break;
        }
        let next = current - value / deriv;
        if !next.is_finite() {
            break;
        }
        current = next;
        value = eval_at(current);
        if value.abs() <= tol {
            return current;
        }
    }

    fallback_bisect(&eval_at, current, 1.0, 20, tol, 60)
}

fn benchmark_evaluate(c: &mut Criterion) {
    let arena = parse_expr("sin(x) + x^2 - 4");
    let base: HashMap<String, f64> = HashMap::new();
    let xs: Vec<f64> = (0..1024).map(|i| -2.0 + (i as f64) * 0.01).collect();

    c.bench_function("fallback_eval_scalar_batch", |b| {
        b.iter(|| {
            let mut acc = 0.0;
            for &x in &xs {
                acc += interpreter::eval_with_var(&arena, &base, "x", x);
            }
            black_box(acc)
        })
    });
}

fn benchmark_derivative(c: &mut Criterion) {
    let arena = parse_expr("cos(x) - x");
    let base: HashMap<String, f64> = HashMap::new();
    let xs: Vec<f64> = (0..512).map(|i| -3.0 + (i as f64) * 0.0125).collect();

    c.bench_function("finite_diff_derivative_batch", |b| {
        b.iter(|| {
            let mut acc = 0.0;
            for &x in &xs {
                acc += interpreter::derivative_with_var(&arena, &base, "x", x, 1e-6);
            }
            black_box(acc)
        })
    });
}

fn benchmark_gradient(c: &mut Criterion) {
    let arena = parse_expr("x * y + y * z + z * x");
    let mut base: HashMap<String, f64> = HashMap::new();
    base.insert("x".to_string(), 1.5);
    base.insert("y".to_string(), -0.75);
    base.insert("z".to_string(), 2.25);
    let vars = vec!["x".to_string(), "y".to_string(), "z".to_string()];

    c.bench_function("finite_diff_gradient", |b| {
        b.iter(|| {
            let grads = interpreter::gradient(&arena, &base, &vars, 1e-6);
            black_box(grads)
        })
    });
}

fn benchmark_newton(c: &mut Criterion) {
    let arena = parse_expr("cos(x) - x");
    let base: HashMap<String, f64> = HashMap::new();
    let guesses: Vec<f64> = (-10..=10).map(|i| i as f64 * 0.5).collect();

    c.bench_function("newton_with_bisection_fallback", |b| {
        b.iter(|| {
            let mut acc = 0.0;
            for &guess in &guesses {
                let root = newton_with_fallback(&arena, &base, guess, 1e-9, 25);
                acc += root;
            }
            black_box(acc)
        })
    });
}

criterion_group!(
    benches,
    benchmark_evaluate,
    benchmark_derivative,
    benchmark_gradient,
    benchmark_newton
);
criterion_main!(benches);
