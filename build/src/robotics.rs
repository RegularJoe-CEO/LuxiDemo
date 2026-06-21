use crate::engine::*;
use crate::eval::hash_f64_slice;

pub fn run_demo() {
    println!();
    println!("  Edge Robotics FSD Demo");
    println!("  ============================================================");
    let values = [0.1f64, 0.5, 1.0, 1.5, 2.0];
    let sigma = 1.0f32;

    let cases: Vec<(&str, Vec<f64>)> = vec![
        (
            "2-link IK",
            values
                .iter()
                .map(|&x| {
                    let t = x as f32;
                    cos_f32_deterministic((t * t - 1.0) / 2.0) as f64
                })
                .collect(),
        ),
        (
            "Kalman gain weight",
            values
                .iter()
                .map(|&x| exp_f32_deterministic(-(x as f32 * x as f32) / (2.0 * sigma * sigma)) as f64)
                .collect(),
        ),
        (
            "Tire slip angle",
            values
                .iter()
                .map(|&x| {
                    let t = x as f32;
                    let denom = 1.0 + t * t;
                    (t / denom).atan() as f64
                })
                .collect(),
        ),
        (
            "Motor torque curve",
            values
                .iter()
                .map(|&x| {
                    let t = x as f32;
                    let s = sin_f32_deterministic(t);
                    (s * s * (1.0 - exp_f32_deterministic(-t / 0.5))) as f64
                })
                .collect(),
        ),
        (
            "Clothoid curvature",
            values
                .iter()
                .map(|&x| {
                    let t = x as f32;
                    if t.abs() < 1e-6 {
                        0.0
                    } else {
                        (sin_f32_deterministic(t * t) / t) as f64
                    }
                })
                .collect(),
        ),
        (
            "Lidar point distance",
            values
                .iter()
                .map(|&x| sqrt_f32_deterministic(x as f32 * x as f32 + 1.0) as f64)
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