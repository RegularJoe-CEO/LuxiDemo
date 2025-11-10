// src/energy.rs — Energy efficiency calculations for SIMD operations
//
// Quantifies operations per joule (ops/J) for different hardware platforms,
// critical for edge deployments (Raspberry Pi 5, Jetson, AWS Graviton) and
// data center energy optimization.

use std::f64;

/// Hardware platform energy profiles
#[derive(Debug, Clone)]
pub struct EnergyProfile {
    /// Platform identifier (e.g., "Pi5", "Jetson Orin", "AWS Graviton3")
    pub platform: String,
    /// Average power draw during SIMD computation (watts)
    pub power_watts: f64,
    /// Idle power consumption (watts)
    pub idle_power_watts: f64,
    /// SIMD width (number of f64 elements processed in parallel)
    pub simd_width: usize,
    /// Clock frequency (MHz)
    pub clock_mhz: f64,
}

/// Energy efficiency metrics
#[derive(Debug, Clone)]
pub struct EnergyMetrics {
    /// Operations per second
    pub ops_per_second: f64,
    /// Power consumption during measurement (watts)
    pub power_watts: f64,
    /// Operations per joule (ops/J)
    pub ops_per_joule: f64,
    /// Energy per operation (nanojoules)
    pub nanojoules_per_op: f64,
    /// Measurement duration (seconds)
    pub duration_seconds: f64,
    /// Total energy consumed (joules)
    pub total_energy_joules: f64,
}

impl EnergyMetrics {
    /// Calculate energy metrics from throughput and power measurements
    pub fn from_measurements(
        ops_per_second: f64,
        power_watts: f64,
        duration_seconds: f64,
    ) -> Self {
        let ops_per_joule = ops_per_second / power_watts;
        let nanojoules_per_op = 1e9 / ops_per_joule;
        let total_energy_joules = power_watts * duration_seconds;

        Self {
            ops_per_second,
            power_watts,
            ops_per_joule,
            nanojoules_per_op,
            duration_seconds,
            total_energy_joules,
        }
    }
}

/// ARM Neon energy profiles for common platforms
pub mod neon_profiles {
    use super::EnergyProfile;

    /// Raspberry Pi 5 (ARM Cortex-A76, 2.4 GHz, 4-core)
    /// NEON SIMD: 128-bit (2x f64)
    pub fn raspberry_pi5() -> EnergyProfile {
        EnergyProfile {
            platform: "Raspberry Pi 5 (Cortex-A76)".to_string(),
            power_watts: 3.0,  // Typical under CPU load
            idle_power_watts: 1.2,
            simd_width: 2,  // 2x f64 per NEON register
            clock_mhz: 2400.0,
        }
    }

    /// NVIDIA Jetson Orin Nano (ARM Cortex-A78AE, 2.0 GHz, 6-core)
    /// NEON SIMD: 128-bit (2x f64)
    pub fn jetson_orin_nano() -> EnergyProfile {
        EnergyProfile {
            platform: "Jetson Orin Nano (Cortex-A78AE)".to_string(),
            power_watts: 7.0,  // 7W TDP mode
            idle_power_watts: 2.0,
            simd_width: 2,
            clock_mhz: 2000.0,
        }
    }

    /// AWS Graviton3 (Neoverse V1, 2.6 GHz)
    /// NEON SIMD: 128-bit (2x f64)
    pub fn aws_graviton3() -> EnergyProfile {
        EnergyProfile {
            platform: "AWS Graviton3 (Neoverse V1)".to_string(),
            power_watts: 5.0,  // Estimated per core
            idle_power_watts: 1.5,
            simd_width: 2,
            clock_mhz: 2600.0,
        }
    }

    /// Apple M2 (ARM, 3.5 GHz performance cores)
    /// NEON SIMD: 128-bit (2x f64)
    pub fn apple_m2() -> EnergyProfile {
        EnergyProfile {
            platform: "Apple M2 (Performance Cores)".to_string(),
            power_watts: 15.0,  // Performance cores under load
            idle_power_watts: 0.5,
            simd_width: 2,
            clock_mhz: 3500.0,
        }
    }

    /// Generic ARM64 fallback
    pub fn generic_arm64() -> EnergyProfile {
        EnergyProfile {
            platform: "Generic ARM64 (NEON)".to_string(),
            power_watts: 5.0,
            idle_power_watts: 1.0,
            simd_width: 2,
            clock_mhz: 2000.0,
        }
    }
}

/// x86_64 AVX2/AVX-512 energy profiles
pub mod x86_profiles {
    use super::EnergyProfile;

    /// Intel Xeon (Cascade Lake, AVX-512)
    /// AVX-512: 512-bit (8x f64)
    pub fn intel_xeon_cascadelake() -> EnergyProfile {
        EnergyProfile {
            platform: "Intel Xeon (Cascade Lake, AVX-512)".to_string(),
            power_watts: 50.0,  // Per socket under AVX-512 load
            idle_power_watts: 15.0,
            simd_width: 8,  // 8x f64 per AVX-512 register
            clock_mhz: 2500.0,
        }
    }

    /// AMD EPYC 7763 (Milan, AVX2)
    /// AVX2: 256-bit (4x f64)
    pub fn amd_epyc_milan() -> EnergyProfile {
        EnergyProfile {
            platform: "AMD EPYC 7763 (Milan, AVX2)".to_string(),
            power_watts: 45.0,  // Per socket
            idle_power_watts: 20.0,
            simd_width: 4,  // 4x f64 per AVX2 register
            clock_mhz: 2450.0,
        }
    }
}

/// Calculate theoretical peak ops/joule for a platform
/// 
/// This provides an upper bound assuming perfect IPC and no memory bottlenecks.
/// Real-world performance will be lower due to cache misses, dependencies, etc.
pub fn theoretical_peak_ops_per_joule(profile: &EnergyProfile) -> f64 {
    // Peak theoretical throughput (ops/sec)
    // = clock_freq (MHz) * 1e6 (cycles/sec) * simd_width (ops/cycle)
    let clock_hz = profile.clock_mhz * 1e6;
    let peak_ops_per_sec = clock_hz * (profile.simd_width as f64);
    
    // Compute power = total power - idle power
    let compute_power = profile.power_watts - profile.idle_power_watts;
    
    if compute_power <= 0.0 {
        return 0.0;
    }
    
    peak_ops_per_sec / compute_power
}

/// Calculate energy efficiency bounds for probabilistic analysis
/// 
/// Returns (pessimistic_ops_per_joule, realistic_ops_per_joule, optimistic_ops_per_joule)
/// based on typical SIMD utilization factors
pub fn energy_efficiency_bounds(profile: &EnergyProfile) -> (f64, f64, f64) {
    let peak = theoretical_peak_ops_per_joule(profile);
    
    // Pessimistic: 20% utilization (memory-bound, cache misses)
    let pessimistic = peak * 0.20;
    
    // Realistic: 50% utilization (typical SIMD workload)
    let realistic = peak * 0.50;
    
    // Optimistic: 80% utilization (well-optimized, cache-friendly)
    let optimistic = peak * 0.80;
    
    (pessimistic, realistic, optimistic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_metrics_calculation() {
        let metrics = EnergyMetrics::from_measurements(1_000_000.0, 5.0, 10.0);
        
        assert_eq!(metrics.ops_per_second, 1_000_000.0);
        assert_eq!(metrics.power_watts, 5.0);
        assert_eq!(metrics.ops_per_joule, 200_000.0);
        assert_eq!(metrics.total_energy_joules, 50.0);
        // nanojoules_per_op = 1e9 / ops_per_joule = 1e9 / 200_000 = 5000 nJ
        assert!((metrics.nanojoules_per_op - 5000.0).abs() < 0.01);
    }

    #[test]
    fn test_pi5_energy_profile() {
        let pi5 = neon_profiles::raspberry_pi5();
        assert_eq!(pi5.simd_width, 2);
        assert!(pi5.power_watts > 0.0);
        
        let peak = theoretical_peak_ops_per_joule(&pi5);
        assert!(peak > 0.0);
        
        // Verify peak is reasonable for Pi5
        // ~2.4 GHz * 2 width / 1.8W compute = ~2.67B ops/J peak theoretical
        assert!(peak > 1_000_000_000.0);  // At least 1B ops/J
        assert!(peak < 5_000_000_000.0); // Less than 5B ops/J
    }

    #[test]
    fn test_energy_efficiency_bounds() {
        let pi5 = neon_profiles::raspberry_pi5();
        let (pessimistic, realistic, optimistic) = energy_efficiency_bounds(&pi5);
        
        assert!(pessimistic < realistic);
        assert!(realistic < optimistic);
        assert!(pessimistic > 0.0);
    }

    #[test]
    fn test_x86_vs_arm_comparison() {
        let pi5 = neon_profiles::raspberry_pi5();
        let xeon = x86_profiles::intel_xeon_cascadelake();
        
        let pi5_peak = theoretical_peak_ops_per_joule(&pi5);
        let xeon_peak = theoretical_peak_ops_per_joule(&xeon);
        
        // ARM should be more energy efficient per operation
        // (though x86 has higher absolute throughput)
        assert!(pi5_peak > xeon_peak * 0.1);  // At least within an order of magnitude
    }
}
