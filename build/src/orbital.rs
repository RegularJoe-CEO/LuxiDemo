use crate::engine::*;
use crate::eval::hash_f64_slice;

pub fn run_demo() {
    println!();
    println!("  Orbital Mechanics Demo");
    println!("  ============================================================");
    let values = [0.1f64, 0.5, 1.0, 2.0, 3.14];

    let cases: Vec<(&str, Vec<f64>)> = vec![
        (
            "Kepler (E - e*sin(E))",
            values
                .iter()
                .map(|&x| (x as f32 - 0.5 * sin_f32_deterministic(x as f32)) as f64)
                .collect(),
        ),
        (
            "Vis-Viva velocity",
            values
                .iter()
                .map(|&x| {
                    let a = 1.0f32;
                    sqrt_f32_deterministic((2.0 / x as f32) - (1.0 / a)) as f64
                })
                .collect(),
        ),
        (
            "Hohmann transfer dV",
            values
                .iter()
                .map(|&x| {
                    let r = x as f32;
                    sqrt_f32_deterministic(r) as f64
                        * (sqrt_f32_deterministic(2.0 / (1.0 + r)) as f64 - 1.0)
                })
                .collect(),
        ),
        (
            "Drag decay exp(-h/H)",
            values
                .iter()
                .map(|&x| exp_f32_deterministic(-x as f32 / 8.5) as f64)
                .collect(),
        ),
        (
            "J2 harmonic",
            values
                .iter()
                .map(|&x| {
                    let c = cos_f32_deterministic(x as f32);
                    (1.5 * c as f64 * c as f64 - 0.5)
                })
                .collect(),
        ),
        (
            "Lambert TOF approx",
            values
                .iter()
                .map(|&x| {
                    let t = sqrt_f32_deterministic(x as f32);
                    ((1.0 - cos_f32_deterministic(t)) / x as f32) as f64
                })
                .collect(),
        ),
    ];

    for (name, results) in cases {
        let hash = hash_f64_slice(&results);
        let short = if hash.len() > 16 {
            format!("{}...", &hash[..16])
        } else {
            hash
        };
        println!("  {:<28} SHA256: {}", name, short);
    }
    println!("  ============================================================");
    println!();
}