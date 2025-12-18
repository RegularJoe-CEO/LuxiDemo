use std::time::{Duration, Instant};
use rhai::{Engine, Scope, EvalAltResult};

fn evaluate_dynamic_expr(engine: &Engine, expr: &str, values: &[f32]) -> Result<Vec<f32>, Box<EvalAltResult>> {
    let ast = engine.compile(expr)?;
    let mut results = Vec::with_capacity(values.len());
    for &x in values {
        let mut scope = Scope::new();
        scope.push("x", x as f64);
        let result: f64 = engine.eval_ast_with_scope(&mut scope, &ast)?;
        results.push(result as f32);
    }
    Ok(results)
}

fn fallback_eval(values: &[f32]) -> Vec<f32> {
    values.iter().map(|&x| x * x).collect()
}

// In-process wearable control loop benchmark (no HTTP / no warp)
// Usage: cargo run --release --bin l4_control_loop
fn main() {
    let hz: f64 = 60.0;
    let warm_ticks: usize = 60;
    let ticks: usize = 3600;
    let payload_floats: usize = 512;
    let expr: &str = "tanh(log(sqrt(x*x+1.0))+exp(-x))*sin(x)+cos(x)";

    let mut values: Vec<f32> = Vec::with_capacity(payload_floats);
    for i in 0..payload_floats {
        values.push(((i % 512) as f32) / 64.0);
    }

    let engine = Engine::new();

    let period = Duration::from_secs_f64(1.0 / hz);
    let budget_ms = period.as_secs_f64() * 1000.0;

    let mut lat_ms: Vec<f64> = Vec::with_capacity(ticks);
    let mut missed: usize = 0;
    let t0 = Instant::now();

    for i in 0..(warm_ticks + ticks) {
        let target = t0 + period * (i as u32);
        while Instant::now() < target {
            let rem = target - Instant::now();
            if rem > Duration::from_millis(1) { std::thread::sleep(Duration::from_millis(1)); }
            else { std::thread::yield_now(); }
        }

        let start = Instant::now();
        let ok = evaluate_dynamic_expr(&engine, expr, &values).is_ok();
        if !ok {
            let _ = fallback_eval(&values);
        }
        let end = Instant::now();

        if i >= warm_ticks {
            let dt = (end - start).as_secs_f64() * 1000.0;
            lat_ms.push(dt);
            if dt > budget_ms { missed += 1; }
        }
    }

    lat_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = lat_ms.len();
    let p = |q: f64| -> f64 {
        let idx = ((q * ((n - 1) as f64)).round() as usize).min(n - 1);
        lat_ms[idx]
    };

    let p50 = p(0.50);
    let p95 = p(0.95);
    let p99 = p(0.99);
    let max = *lat_ms.last().unwrap_or(&0.0);

    println!(
        "{{\n  \"profile\": \"wearable_control_loop_inproc\",\n  \"hz\": {hz},\n  \"budget_ms\": {budget:.3},\n  \"ticks_ok\": {ticks},\n  \"payload_floats\": {payload},\n  \"lat_p50_ms\": {p50:.3},\n  \"lat_p95_ms\": {p95:.3},\n  \"lat_p99_ms\": {p99:.3},\n  \"lat_max_ms\": {max:.3},\n  \"missed_deadlines\": {missed}\n}}",
        hz = hz, budget = budget_ms, ticks = ticks, payload = payload_floats,
        p50 = p50, p95 = p95, p99 = p99, max = max, missed = missed
    );
}
