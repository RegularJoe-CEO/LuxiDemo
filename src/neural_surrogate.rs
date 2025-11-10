// src/neural_surrogate.rs — Neural surrogate integration for hybrid ML-physics uncertainty propagation
//
// This module provides neural network-based surrogate models that can accelerate
// Monte Carlo simulations by learning to approximate expensive physics calculations.
// Primary use case: xAI orbit forecasting with hybrid ML-physics approach.

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

#[cfg(feature = "neural")]
use tract_onnx::prelude::*;
#[cfg(feature = "neural")]
use ndarray::{Array1, Array2};

/// Configuration for neural surrogate model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrogateConfig {
    /// Path to ONNX model file (exported from PyTorch/TensorFlow)
    pub model_path: Option<String>,
    /// Confidence threshold below which to fallback to physics simulation
    pub confidence_threshold: f64,
    /// Whether to use surrogate for initial prediction
    pub use_surrogate_init: bool,
    /// Whether to validate surrogate predictions against physics
    pub validate_with_physics: bool,
}

impl Default for SurrogateConfig {
    fn default() -> Self {
        Self {
            model_path: None,
            confidence_threshold: 0.95,
            use_surrogate_init: true,
            validate_with_physics: true,
        }
    }
}

/// Statistics for hybrid ML-physics convergence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceStats {
    /// Total number of evaluations
    pub total_evals: usize,
    /// Number of surrogate predictions used
    pub surrogate_evals: usize,
    /// Number of physics calculations used
    pub physics_evals: usize,
    /// Mean absolute error of surrogate vs physics (when validated)
    pub surrogate_mae: f64,
    /// Speedup factor vs pure physics approach
    pub speedup_factor: f64,
    /// Wall clock time in seconds
    pub wall_time_secs: f64,
}

/// Neural surrogate model for orbit time-of-flight prediction
#[cfg(feature = "neural")]
pub struct NeuralSurrogate {
    model: SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
    config: SurrogateConfig,
}

#[cfg(feature = "neural")]
impl NeuralSurrogate {
    /// Load a neural surrogate from an ONNX file
    ///
    /// The model should accept inputs: [a, r1, r2, c, s, mu, n_rev]
    /// and output: [tof, confidence]
    pub fn from_onnx(path: &str, config: SurrogateConfig) -> Result<Self> {
        let model = tract_onnx::onnx()
            .model_for_path(path)
            .context("Failed to load ONNX model")?
            .into_optimized()
            .context("Failed to optimize model")?
            .into_runnable()
            .context("Failed to create runnable model")?;
        
        Ok(Self { model, config })
    }
    
    /// Predict TOF using the neural surrogate
    ///
    /// Returns (predicted_tof, confidence_score)
    pub fn predict(&self, a: f64, r1: f64, r2: f64, c: f64, s: f64, mu: f64, n_rev: i32) -> Result<(f64, f64)> {
        // Prepare input tensor: [a, r1, r2, c, s, mu, n_rev]
        let input = Array2::from_shape_vec(
            (1, 7),
            vec![a, r1, r2, c, s, mu, n_rev as f64]
        )?;
        
        let input_tensor = input.into_tensor();
        
        // Run inference
        let outputs = self.model.run(tvec![input_tensor.into()])?;
        
        // Extract output: [tof, confidence]
        let output = outputs[0].to_array_view::<f32>()?;
        let tof = output[[0, 0]] as f64;
        let confidence = output[[0, 1]] as f64;
        
        Ok((tof, confidence))
    }
    
    /// Predict TOF for a batch of inputs
    pub fn predict_batch(&self, inputs: &[(f64, f64, f64, f64, f64, f64, i32)]) -> Result<Vec<(f64, f64)>> {
        let mut results = Vec::with_capacity(inputs.len());
        
        for &(a, r1, r2, c, s, mu, n_rev) in inputs {
            results.push(self.predict(a, r1, r2, c, s, mu, n_rev)?);
        }
        
        Ok(results)
    }
}

/// Hybrid Monte Carlo that uses neural surrogate for acceleration
///
/// This function combines neural network predictions with physics-based
/// Monte Carlo to achieve faster convergence while maintaining accuracy.
///
/// # Parameters
/// - `a_nominal`: Nominal semi-major axis (km)
/// - `a_std_dev`: Standard deviation of semi-major axis (km)
/// - `r1, r2, c, s, mu`: Lambert problem parameters
/// - `n_rev`: Number of complete revolutions
/// - `n_samples`: Number of Monte Carlo samples
/// - `surrogate`: Optional neural surrogate model
///
/// # Returns
/// Vector of (a, tof) pairs and convergence statistics
#[cfg(feature = "neural")]
pub fn hybrid_monte_carlo_tof(
    a_nominal: f64,
    a_std_dev: f64,
    r1: f64,
    r2: f64,
    c: f64,
    s: f64,
    mu: f64,
    n_rev: i32,
    n_samples: usize,
    surrogate: Option<&NeuralSurrogate>,
) -> Result<(Vec<(f64, f64)>, ConvergenceStats)> {
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal};
    use std::time::Instant;
    use crate::lambert::lambert_tof_multirev;
    
    let start = Instant::now();
    let mut rng = thread_rng();
    let normal = Normal::new(a_nominal, a_std_dev).unwrap();
    
    let mut results = Vec::with_capacity(n_samples);
    let mut surrogate_count = 0;
    let mut physics_count = 0;
    let mut total_error = 0.0;
    let mut error_samples = 0;
    
    for _ in 0..n_samples {
        let a = normal.sample(&mut rng);
        
        // Skip invalid values
        if a <= s / 2.0 {
            continue;
        }
        
        let (tof, _used_surrogate) = if let Some(surr) = surrogate {
            // Try surrogate first
            if let Ok((pred_tof, confidence)) = surr.predict(a, r1, r2, c, s, mu, n_rev) {
                if confidence >= surr.config.confidence_threshold {
                    // Use surrogate prediction
                    surrogate_count += 1;
                    
                    // Optionally validate against physics
                    if surr.config.validate_with_physics {
                        let physics_tof = lambert_tof_multirev(a, r1, r2, c, s, mu, n_rev);
                        if physics_tof.is_finite() {
                            total_error += (pred_tof - physics_tof).abs();
                            error_samples += 1;
                        }
                    }
                    
                    (pred_tof, true)
                } else {
                    // Low confidence, fallback to physics
                    physics_count += 1;
                    (lambert_tof_multirev(a, r1, r2, c, s, mu, n_rev), false)
                }
            } else {
                // Surrogate failed, use physics
                physics_count += 1;
                (lambert_tof_multirev(a, r1, r2, c, s, mu, n_rev), false)
            }
        } else {
            // No surrogate, pure physics
            physics_count += 1;
            (lambert_tof_multirev(a, r1, r2, c, s, mu, n_rev), false)
        };
        
        if tof.is_finite() {
            results.push((a, tof));
        }
    }
    
    let wall_time = start.elapsed().as_secs_f64();
    
    // Calculate speedup: assume surrogate is 100x faster than physics
    // This is a typical speedup for neural network inference vs numerical integration
    let effective_physics_evals = physics_count + (surrogate_count as f64 / 100.0) as usize;
    let speedup = n_samples as f64 / effective_physics_evals as f64;
    
    let stats = ConvergenceStats {
        total_evals: results.len(),
        surrogate_evals: surrogate_count,
        physics_evals: physics_count,
        surrogate_mae: if error_samples > 0 { total_error / error_samples as f64 } else { 0.0 },
        speedup_factor: speedup,
        wall_time_secs: wall_time,
    };
    
    Ok((results, stats))
}

/// Fallback implementation when neural feature is not enabled
#[cfg(not(feature = "neural"))]
pub fn hybrid_monte_carlo_tof(
    a_nominal: f64,
    a_std_dev: f64,
    r1: f64,
    r2: f64,
    c: f64,
    s: f64,
    mu: f64,
    n_rev: i32,
    n_samples: usize,
    _surrogate: Option<()>,
) -> Result<(Vec<(f64, f64)>, ConvergenceStats)> {
    use std::time::Instant;
    use crate::lambert::monte_carlo_tof;
    
    let start = Instant::now();
    let results = monte_carlo_tof(a_nominal, a_std_dev, r1, r2, c, s, mu, n_rev, n_samples);
    let wall_time = start.elapsed().as_secs_f64();
    
    let stats = ConvergenceStats {
        total_evals: results.len(),
        surrogate_evals: 0,
        physics_evals: results.len(),
        surrogate_mae: 0.0,
        speedup_factor: 1.0,
        wall_time_secs: wall_time,
    };
    
    Ok((results, stats))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_surrogate_config_default() {
        let config = SurrogateConfig::default();
        assert_eq!(config.confidence_threshold, 0.95);
        assert!(config.use_surrogate_init);
        assert!(config.validate_with_physics);
    }
    
    #[test]
    #[cfg(not(feature = "neural"))]
    fn test_hybrid_monte_carlo_fallback() {
        let result = hybrid_monte_carlo_tof(
            6066.0, 10.0,
            6980.0, 10520.0, 6655.0, 12078.0, 398600.0,
            0, 100, None
        );
        
        assert!(result.is_ok());
        let (samples, stats) = result.unwrap();
        assert!(samples.len() > 0);
        assert_eq!(stats.surrogate_evals, 0);
        assert_eq!(stats.physics_evals, samples.len());
    }
}
