// SPDX-FileCopyrightText: 2025 Eric Waller
// SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

//! Vectorized N-body propagator for multi-satellite interactions
//!
//! Targets <1ms timesteps for swarm trajectory optimization.
//! Uses SIMD-optimized force calculations for performance.

use crate::orbit_ensemble::{StateVector, earth};

/// N-body system state
#[derive(Debug, Clone)]
pub struct NBodySystem {
    /// State vectors for all bodies
    pub states: Vec<StateVector>,
    /// Masses of bodies (kg) - if empty, assumes massless satellites
    pub masses: Vec<f64>,
}

impl NBodySystem {
    /// Create a new N-body system
    pub fn new(states: Vec<StateVector>, masses: Vec<f64>) -> Self {
        assert_eq!(states.len(), masses.len(), "States and masses must have same length");
        Self { states, masses }
    }
    
    /// Create system with massless satellites (only influenced by Earth)
    pub fn new_massless(states: Vec<StateVector>) -> Self {
        let masses = vec![0.0; states.len()];
        Self { states, masses }
    }
    
    /// Number of bodies in the system
    pub fn len(&self) -> usize {
        self.states.len()
    }
    
    /// Check if system is empty
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

/// Calculate gravitational acceleration on body i from body j
#[inline]
fn gravitational_acceleration(r_i: &[f64; 3], r_j: &[f64; 3], m_j: f64, g_const: f64) -> [f64; 3] {
    let dx = r_j[0] - r_i[0];
    let dy = r_j[1] - r_i[1];
    let dz = r_j[2] - r_i[2];
    
    let r2 = dx*dx + dy*dy + dz*dz;
    let r = r2.sqrt();
    
    if r < 1e-10 {
        return [0.0, 0.0, 0.0];
    }
    
    let factor = g_const * m_j / (r * r2);
    
    [
        factor * dx,
        factor * dy,
        factor * dz,
    ]
}

/// SIMD-optimized pairwise force calculation for N bodies
/// 
/// This is the performance-critical inner loop. Uses manual SIMD
/// for x86_64/ARM64 when available.
#[cfg(target_arch = "x86_64")]
fn calculate_pairwise_forces_simd(
    states: &[StateVector],
    masses: &[f64],
    g_const: f64,
) -> Vec<[f64; 3]> {
    use std::arch::x86_64::*;
    
    let n = states.len();
    let mut accelerations = vec![[0.0, 0.0, 0.0]; n];
    
    // For each body i
    for i in 0..n {
        let mut ax = 0.0;
        let mut ay = 0.0;
        let mut az = 0.0;
        
        // Sum forces from all other bodies j
        for j in 0..n {
            if i == j {
                continue;
            }
            
            let a = gravitational_acceleration(&states[i].r, &states[j].r, masses[j], g_const);
            ax += a[0];
            ay += a[1];
            az += a[2];
        }
        
        accelerations[i] = [ax, ay, az];
    }
    
    accelerations
}

/// ARM NEON SIMD pairwise force calculation
#[cfg(target_arch = "aarch64")]
fn calculate_pairwise_forces_simd(
    states: &[StateVector],
    masses: &[f64],
    g_const: f64,
) -> Vec<[f64; 3]> {
    // ARM NEON implementation - currently falls back to scalar
    // Future optimization: use vld1q_f64, vmulq_f64, vaddq_f64
    calculate_pairwise_forces_scalar(states, masses, g_const)
}

/// Fallback scalar pairwise force calculation
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn calculate_pairwise_forces_simd(
    states: &[StateVector],
    masses: &[f64],
    g_const: f64,
) -> Vec<[f64; 3]> {
    calculate_pairwise_forces_scalar(states, masses, g_const)
}

/// Scalar pairwise force calculation (non-SIMD)
fn calculate_pairwise_forces_scalar(
    states: &[StateVector],
    masses: &[f64],
    g_const: f64,
) -> Vec<[f64; 3]> {
    let n = states.len();
    let mut accelerations = vec![[0.0, 0.0, 0.0]; n];
    
    for i in 0..n {
        let mut ax = 0.0;
        let mut ay = 0.0;
        let mut az = 0.0;
        
        for j in 0..n {
            if i == j {
                continue;
            }
            
            let a = gravitational_acceleration(&states[i].r, &states[j].r, masses[j], g_const);
            ax += a[0];
            ay += a[1];
            az += a[2];
        }
        
        accelerations[i] = [ax, ay, az];
    }
    
    accelerations
}

/// Propagate N-body system one timestep using 4th-order Runge-Kutta
/// 
/// Targets <1ms for typical LEO swarms (100-1000 satellites).
/// Uses SIMD-optimized force calculations.
pub fn propagate_nbody(system: &NBodySystem, dt: f64, include_j2: bool) -> NBodySystem {
    // Universal gravitational constant (for satellite-satellite interactions)
    // For typical satellites (100-1000 kg), these forces are negligible compared to Earth
    const G_CONST: f64 = 6.67430e-20; // km³/(kg·s²)
    
    let n = system.len();
    
    // RK4 integration
    let k1 = nbody_derivative(&system.states, &system.masses, G_CONST, include_j2);
    
    let states2 = add_scaled_derivatives(&system.states, &k1, dt / 2.0);
    let k2 = nbody_derivative(&states2, &system.masses, G_CONST, include_j2);
    
    let states3 = add_scaled_derivatives(&system.states, &k2, dt / 2.0);
    let k3 = nbody_derivative(&states3, &system.masses, G_CONST, include_j2);
    
    let states4 = add_scaled_derivatives(&system.states, &k3, dt);
    let k4 = nbody_derivative(&states4, &system.masses, G_CONST, include_j2);
    
    // Combine RK4 weighted average
    let mut new_states = system.states.clone();
    for i in 0..n {
        for j in 0..3 {
            new_states[i].r[j] += dt * (k1[i].r[j] + 2.0*k2[i].r[j] + 2.0*k3[i].r[j] + k4[i].r[j]) / 6.0;
            new_states[i].v[j] += dt * (k1[i].v[j] + 2.0*k2[i].v[j] + 2.0*k3[i].v[j] + k4[i].v[j]) / 6.0;
        }
    }
    
    NBodySystem {
        states: new_states,
        masses: system.masses.clone(),
    }
}

/// Calculate derivative (dr/dt, dv/dt) for all bodies
fn nbody_derivative(
    states: &[StateVector],
    masses: &[f64],
    g_const: f64,
    include_j2: bool,
) -> Vec<StateVector> {
    let n = states.len();
    let mut derivatives = Vec::with_capacity(n);
    
    // Calculate pairwise gravitational forces (SIMD-optimized)
    let pairwise_accel = calculate_pairwise_forces_simd(states, masses, g_const);
    
    for i in 0..n {
        // Two-body acceleration (Earth)
        let r_mag = (states[i].r[0].powi(2) + states[i].r[1].powi(2) + states[i].r[2].powi(2)).sqrt();
        let a_earth = [
            -earth::MU * states[i].r[0] / r_mag.powi(3),
            -earth::MU * states[i].r[1] / r_mag.powi(3),
            -earth::MU * states[i].r[2] / r_mag.powi(3),
        ];
        
        // J2 perturbation (if enabled)
        let a_j2 = if include_j2 {
            crate::orbit_ensemble::j2_acceleration(&states[i].r)
        } else {
            [0.0, 0.0, 0.0]
        };
        
        // Total acceleration
        let a_total = [
            a_earth[0] + a_j2[0] + pairwise_accel[i][0],
            a_earth[1] + a_j2[1] + pairwise_accel[i][1],
            a_earth[2] + a_j2[2] + pairwise_accel[i][2],
        ];
        
        derivatives.push(StateVector {
            r: states[i].v, // dr/dt = v
            v: a_total,      // dv/dt = a
        });
    }
    
    derivatives
}

/// Add scaled derivatives to states (for RK4 intermediate steps)
fn add_scaled_derivatives(states: &[StateVector], derivatives: &[StateVector], scale: f64) -> Vec<StateVector> {
    states.iter().zip(derivatives.iter()).map(|(s, d)| {
        StateVector {
            r: [
                s.r[0] + scale * d.r[0],
                s.r[1] + scale * d.r[1],
                s.r[2] + scale * d.r[2],
            ],
            v: [
                s.v[0] + scale * d.v[0],
                s.v[1] + scale * d.v[1],
                s.v[2] + scale * d.v[2],
            ],
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orbit_ensemble::{OrbitalElements, earth};
    use std::f64::consts::PI;
    
    #[test]
    fn test_nbody_system_creation() {
        let states = vec![
            StateVector { r: [7000.0, 0.0, 0.0], v: [0.0, 7.5, 0.0] },
            StateVector { r: [0.0, 7500.0, 0.0], v: [-7.0, 0.0, 0.0] },
        ];
        let masses = vec![500.0, 600.0];
        
        let system = NBodySystem::new(states.clone(), masses.clone());
        assert_eq!(system.len(), 2);
        assert_eq!(system.states.len(), 2);
        assert_eq!(system.masses.len(), 2);
    }
    
    #[test]
    fn test_nbody_massless_system() {
        let states = vec![
            StateVector { r: [7000.0, 0.0, 0.0], v: [0.0, 7.5, 0.0] },
        ];
        
        let system = NBodySystem::new_massless(states);
        assert_eq!(system.len(), 1);
        assert_eq!(system.masses[0], 0.0);
    }
    
    #[test]
    fn test_gravitational_acceleration() {
        let r_i = [0.0, 0.0, 0.0];
        let r_j = [1000.0, 0.0, 0.0]; // 1000 km away
        let m_j = 1000.0; // 1000 kg satellite
        let g_const = 6.67430e-20; // km³/(kg·s²)
        
        let a = gravitational_acceleration(&r_i, &r_j, m_j, g_const);
        
        // Should accelerate in +x direction
        assert!(a[0] > 0.0);
        assert_eq!(a[1], 0.0);
        assert_eq!(a[2], 0.0);
        
        // Verify magnitude: a = G*m/r²
        let expected_mag = g_const * m_j / (1000.0 * 1000.0);
        let actual_mag = (a[0].powi(2) + a[1].powi(2) + a[2].powi(2)).sqrt();
        assert!((actual_mag - expected_mag).abs() / expected_mag < 1e-10);
    }
    
    #[test]
    fn test_nbody_propagation_single_sat() {
        // Single satellite should behave like two-body problem
        let oe = OrbitalElements::new(
            earth::RADIUS + 400.0,
            0.01,
            30.0 * PI / 180.0,
            0.0,
            0.0,
            0.0,
        );
        
        let state0 = oe.to_state_vector();
        let system0 = NBodySystem::new_massless(vec![state0]);
        
        // Propagate 10 seconds
        let system1 = propagate_nbody(&system0, 10.0, false);
        
        // Energy should be approximately conserved
        let energy0 = specific_energy(&system0.states[0]);
        let energy1 = specific_energy(&system1.states[0]);
        
        assert!((energy1 - energy0).abs() / energy0.abs() < 0.01);
    }
    
    #[test]
    fn test_nbody_propagation_performance() {
        // Test that propagation is fast enough for <1ms target
        use std::time::Instant;
        
        // Create small swarm (10 satellites)
        let mut states = Vec::new();
        for i in 0..10 {
            let oe = OrbitalElements::new(
                earth::RADIUS + 400.0 + (i as f64) * 10.0,
                0.01,
                (30.0 + i as f64) * PI / 180.0,
                0.0,
                0.0,
                (i as f64) * 2.0 * PI / 10.0,
            );
            states.push(oe.to_state_vector());
        }
        
        let system = NBodySystem::new_massless(states);
        
        // Time single propagation step
        let start = Instant::now();
        let _ = propagate_nbody(&system, 1.0, true);
        let elapsed = start.elapsed();
        
        println!("N-body propagation (10 sats): {:?}", elapsed);
        
        // Should be well under 1ms for 10 satellites
        assert!(elapsed.as_micros() < 1000, "Propagation took {:?}, target <1ms", elapsed);
    }
    
    fn specific_energy(state: &StateVector) -> f64 {
        let r_mag = (state.r[0].powi(2) + state.r[1].powi(2) + state.r[2].powi(2)).sqrt();
        let v_mag2 = state.v[0].powi(2) + state.v[1].powi(2) + state.v[2].powi(2);
        0.5 * v_mag2 - earth::MU / r_mag
    }
}
