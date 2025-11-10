// SPDX-FileCopyrightText: 2025 Eric Waller
// SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

//! Orbital ensemble generation for synthetic benchmarks
//!
//! Generates diverse LEO swarms with J2 perturbations for testing
//! orbital mechanics calculations and convergence analysis.

use std::f64::consts::PI;
use rand::Rng;
use rand::distributions::Uniform;

/// Earth physical constants
pub mod earth {
    /// Gravitational parameter (km³/s²)
    pub const MU: f64 = 398600.4418;
    
    /// Equatorial radius (km)
    pub const RADIUS: f64 = 6378.137;
    
    /// J2 perturbation coefficient (oblateness)
    pub const J2: f64 = 1.08263e-3;
}

/// Orbital elements (Keplerian)
#[derive(Debug, Clone, Copy)]
pub struct OrbitalElements {
    /// Semi-major axis (km)
    pub a: f64,
    /// Eccentricity (dimensionless, 0-1)
    pub e: f64,
    /// Inclination (radians)
    pub i: f64,
    /// Right ascension of ascending node (radians)
    pub omega_lan: f64,
    /// Argument of periapsis (radians)
    pub omega_ap: f64,
    /// Mean anomaly (radians)
    pub m: f64,
}

/// Cartesian state vector (position + velocity)
#[derive(Debug, Clone, Copy)]
pub struct StateVector {
    /// Position (x, y, z) in km
    pub r: [f64; 3],
    /// Velocity (vx, vy, vz) in km/s
    pub v: [f64; 3],
}

impl OrbitalElements {
    /// Create a new orbital element set
    pub fn new(a: f64, e: f64, i: f64, omega_lan: f64, omega_ap: f64, m: f64) -> Self {
        Self { a, e, i, omega_lan, omega_ap, m }
    }
    
    /// Convert orbital elements to Cartesian state vector (position, velocity)
    pub fn to_state_vector(&self) -> StateVector {
        // Solve Kepler's equation for eccentric anomaly
        let e_anom = self.solve_kepler(self.m, self.e, 1e-8, 20);
        
        // True anomaly from eccentric anomaly
        let nu = 2.0 * ((1.0 + self.e).sqrt() * (e_anom / 2.0).tan()).atan2(
            (1.0 - self.e).sqrt() * (e_anom / 2.0).sin() / (e_anom / 2.0).cos()
        );
        
        // Distance from focus
        let r_mag = self.a * (1.0 - self.e * e_anom.cos());
        
        // Position in orbital plane
        let r_orb = [
            r_mag * nu.cos(),
            r_mag * nu.sin(),
            0.0,
        ];
        
        // Velocity in orbital plane
        let v_factor = (earth::MU / self.a).sqrt();
        let v_orb = [
            -v_factor * e_anom.sin() / (1.0 - self.e * e_anom.cos()),
            v_factor * (1.0 - self.e.powi(2)).sqrt() * e_anom.cos() / (1.0 - self.e * e_anom.cos()),
            0.0,
        ];
        
        // Rotate to inertial frame
        let r = self.rotate_to_inertial(r_orb);
        let v = self.rotate_to_inertial(v_orb);
        
        StateVector { r, v }
    }
    
    /// Solve Kepler's equation: M = E - e*sin(E)
    fn solve_kepler(&self, m: f64, e: f64, tol: f64, max_iter: usize) -> f64 {
        let mut e_anom = m; // Initial guess
        for _ in 0..max_iter {
            let delta = (m - e_anom + e * e_anom.sin()) / (1.0 - e * e_anom.cos());
            e_anom += delta;
            if delta.abs() < tol {
                break;
            }
        }
        e_anom
    }
    
    /// Rotate from orbital plane to inertial frame
    fn rotate_to_inertial(&self, v_orb: [f64; 3]) -> [f64; 3] {
        let cos_omega = self.omega_ap.cos();
        let sin_omega = self.omega_ap.sin();
        let cos_i = self.i.cos();
        let sin_i = self.i.sin();
        let cos_lan = self.omega_lan.cos();
        let sin_lan = self.omega_lan.sin();
        
        // Rotation matrix: R = Rz(LAN) * Rx(i) * Rz(omega)
        let r11 = cos_lan * cos_omega - sin_lan * cos_i * sin_omega;
        let r12 = -cos_lan * sin_omega - sin_lan * cos_i * cos_omega;
        let r21 = sin_lan * cos_omega + cos_lan * cos_i * sin_omega;
        let r22 = -sin_lan * sin_omega + cos_lan * cos_i * cos_omega;
        let r31 = sin_i * sin_omega;
        let r32 = sin_i * cos_omega;
        
        [
            r11 * v_orb[0] + r12 * v_orb[1],
            r21 * v_orb[0] + r22 * v_orb[1],
            r31 * v_orb[0] + r32 * v_orb[1],
        ]
    }
}

/// LEO swarm configuration
#[derive(Debug, Clone)]
pub struct LeoSwarmConfig {
    /// Number of satellites
    pub num_sats: usize,
    /// Altitude range (km above Earth surface)
    pub altitude_range: (f64, f64),
    /// Inclination range (radians)
    pub inclination_range: (f64, f64),
    /// Eccentricity range
    pub eccentricity_range: (f64, f64),
}

impl Default for LeoSwarmConfig {
    fn default() -> Self {
        Self {
            num_sats: 1000,
            // LEO: 200-2000 km altitude
            altitude_range: (200.0, 2000.0),
            // Typical LEO: 0-100 degrees inclination
            inclination_range: (0.0, 100.0 * PI / 180.0),
            // Near-circular orbits
            eccentricity_range: (0.0, 0.05),
        }
    }
}

/// Generate a diverse LEO swarm ensemble
pub fn generate_leo_swarm(config: &LeoSwarmConfig) -> Vec<OrbitalElements> {
    let mut rng = rand::thread_rng();
    let mut swarm = Vec::with_capacity(config.num_sats);
    
    let alt_dist = Uniform::new(config.altitude_range.0, config.altitude_range.1);
    let inc_dist = Uniform::new(config.inclination_range.0, config.inclination_range.1);
    let ecc_dist = Uniform::new(config.eccentricity_range.0, config.eccentricity_range.1);
    let angle_dist = Uniform::new(0.0, 2.0 * PI);
    
    for _ in 0..config.num_sats {
        let altitude = rng.sample(alt_dist);
        let a = earth::RADIUS + altitude;
        let e = rng.sample(ecc_dist);
        let i = rng.sample(inc_dist);
        let omega_lan = rng.sample(angle_dist);
        let omega_ap = rng.sample(angle_dist);
        let m = rng.sample(angle_dist);
        
        swarm.push(OrbitalElements::new(a, e, i, omega_lan, omega_ap, m));
    }
    
    swarm
}

/// Calculate J2 perturbation acceleration
/// 
/// J2 is the second zonal harmonic representing Earth's oblateness.
/// This causes precession of the orbital elements over time.
pub fn j2_acceleration(r: &[f64; 3]) -> [f64; 3] {
    let r_mag = (r[0].powi(2) + r[1].powi(2) + r[2].powi(2)).sqrt();
    
    if r_mag < 1e-10 {
        return [0.0, 0.0, 0.0];
    }
    
    // J2 perturbation factor
    let factor = -1.5 * earth::J2 * earth::MU * earth::RADIUS.powi(2) / r_mag.powi(5);
    
    // Zonal harmonic term (depends on z-component)
    let z_factor = 5.0 * r[2].powi(2) / r_mag.powi(2);
    
    [
        factor * r[0] * (z_factor - 1.0),
        factor * r[1] * (z_factor - 1.0),
        factor * r[2] * (z_factor - 3.0),
    ]
}

/// Propagate orbital state with J2 perturbations using RK4 integration
/// 
/// Returns new state after time dt
pub fn propagate_j2(state: &StateVector, dt: f64) -> StateVector {
    // 4th-order Runge-Kutta integration
    let k1 = state_derivative(state);
    
    let state2 = add_scaled_derivative(state, &k1, dt / 2.0);
    let k2 = state_derivative(&state2);
    
    let state3 = add_scaled_derivative(state, &k2, dt / 2.0);
    let k3 = state_derivative(&state3);
    
    let state4 = add_scaled_derivative(state, &k3, dt);
    let k4 = state_derivative(&state4);
    
    // Combine RK4 weighted average
    let mut new_state = *state;
    for i in 0..3 {
        new_state.r[i] += dt * (k1.r[i] + 2.0*k2.r[i] + 2.0*k3.r[i] + k4.r[i]) / 6.0;
        new_state.v[i] += dt * (k1.v[i] + 2.0*k2.v[i] + 2.0*k3.v[i] + k4.v[i]) / 6.0;
    }
    
    new_state
}

/// Calculate state derivative (dr/dt, dv/dt) with J2 perturbations
fn state_derivative(state: &StateVector) -> StateVector {
    let r_mag = (state.r[0].powi(2) + state.r[1].powi(2) + state.r[2].powi(2)).sqrt();
    
    // Two-body acceleration
    let a_twobody = [
        -earth::MU * state.r[0] / r_mag.powi(3),
        -earth::MU * state.r[1] / r_mag.powi(3),
        -earth::MU * state.r[2] / r_mag.powi(3),
    ];
    
    // J2 perturbation
    let a_j2 = j2_acceleration(&state.r);
    
    // Total acceleration
    let a_total = [
        a_twobody[0] + a_j2[0],
        a_twobody[1] + a_j2[1],
        a_twobody[2] + a_j2[2],
    ];
    
    StateVector {
        r: state.v, // dr/dt = v
        v: a_total, // dv/dt = a
    }
}

/// Add scaled derivative to state (for RK4 intermediate steps)
fn add_scaled_derivative(state: &StateVector, deriv: &StateVector, scale: f64) -> StateVector {
    StateVector {
        r: [
            state.r[0] + scale * deriv.r[0],
            state.r[1] + scale * deriv.r[1],
            state.r[2] + scale * deriv.r[2],
        ],
        v: [
            state.v[0] + scale * deriv.v[0],
            state.v[1] + scale * deriv.v[1],
            state.v[2] + scale * deriv.v[2],
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_leo_swarm() {
        let config = LeoSwarmConfig::default();
        let swarm = generate_leo_swarm(&config);
        
        assert_eq!(swarm.len(), config.num_sats);
        
        // Check all satellites are in LEO range
        for sat in &swarm {
            let altitude = sat.a - earth::RADIUS;
            assert!(altitude >= config.altitude_range.0);
            assert!(altitude <= config.altitude_range.1);
            assert!(sat.e >= config.eccentricity_range.0);
            assert!(sat.e <= config.eccentricity_range.1);
        }
    }
    
    #[test]
    fn test_orbital_elements_conversion() {
        // Simple circular equatorial orbit at 400km
        let oe = OrbitalElements::new(
            earth::RADIUS + 400.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        );
        
        let sv = oe.to_state_vector();
        
        // Position should be approximately equal to semi-major axis
        let r_mag = (sv.r[0].powi(2) + sv.r[1].powi(2) + sv.r[2].powi(2)).sqrt();
        assert!((r_mag - oe.a).abs() < 1.0); // Within 1 km
        
        // Velocity for circular orbit: v = sqrt(mu/a)
        let v_mag = (sv.v[0].powi(2) + sv.v[1].powi(2) + sv.v[2].powi(2)).sqrt();
        let v_expected = (earth::MU / oe.a).sqrt();
        assert!((v_mag - v_expected).abs() / v_expected < 0.01); // Within 1%
    }
    
    #[test]
    fn test_j2_perturbation() {
        // Test J2 acceleration at equator vs pole
        let r_eq = [earth::RADIUS + 400.0, 0.0, 0.0];
        let r_pole = [0.0, 0.0, earth::RADIUS + 400.0];
        
        let a_eq = j2_acceleration(&r_eq);
        let a_pole = j2_acceleration(&r_pole);
        
        // Both should have non-zero accelerations
        let a_eq_mag = (a_eq[0].powi(2) + a_eq[1].powi(2) + a_eq[2].powi(2)).sqrt();
        let a_pole_mag = (a_pole[0].powi(2) + a_pole[1].powi(2) + a_pole[2].powi(2)).sqrt();
        
        assert!(a_eq_mag > 0.0);
        assert!(a_pole_mag > 0.0);
    }
    
    #[test]
    fn test_propagation_conservation() {
        // Test energy conservation for short propagation
        let oe = OrbitalElements::new(
            earth::RADIUS + 400.0,
            0.01,
            30.0 * PI / 180.0,
            0.0,
            0.0,
            0.0,
        );
        
        let state0 = oe.to_state_vector();
        let state1 = propagate_j2(&state0, 10.0); // 10 second propagation
        
        // Calculate specific energy (should be approximately conserved)
        let energy0 = specific_energy(&state0);
        let energy1 = specific_energy(&state1);
        
        // Energy should be conserved within 1% for short timesteps
        assert!((energy1 - energy0).abs() / energy0.abs() < 0.01);
    }
    
    fn specific_energy(state: &StateVector) -> f64 {
        let r_mag = (state.r[0].powi(2) + state.r[1].powi(2) + state.r[2].powi(2)).sqrt();
        let v_mag2 = state.v[0].powi(2) + state.v[1].powi(2) + state.v[2].powi(2);
        0.5 * v_mag2 - earth::MU / r_mag
    }
}
