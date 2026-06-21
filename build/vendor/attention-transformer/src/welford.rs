//! Welford's online algorithm for numerically stable mean/variance.
//! Used for deterministic LayerNorm and statistics throughout the transformer.

#[derive(Debug, Clone, Default)]
pub struct WelfordState {
    pub mean: f32,
    pub m2: f32,
    pub count: u32,
}

impl WelfordState {
    pub fn new() -> Self {
        Self {
            mean: 0.0,
            m2: 0.0,
            count: 0,
        }
    }

    #[inline]
    pub fn update(&mut self, x: f32) {
        self.count += 1;
        let delta = x - self.mean;
        self.mean += delta / self.count as f32;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }

    pub fn variance(&self) -> f32 {
        if self.count < 2 {
            0.0
        } else {
            self.m2 / self.count as f32
        }
    }

    pub fn std(&self, eps: f32) -> f32 {
        (self.variance() + eps).sqrt()
    }

    pub fn merge(a: &Self, b: &Self) -> Self {
        let count = a.count + b.count;
        if count == 0 {
            return Self::new();
        }
        let delta = b.mean - a.mean;
        let mean = a.mean + delta * b.count as f32 / count as f32;
        let m2 = a.m2 + b.m2 + delta * delta * a.count as f32 * b.count as f32 / count as f32;
        Self { mean, m2, count }
    }
}

#[cfg(test)]
mod tests {
    use super::WelfordState;

    fn naive_mean_var(data: &[f32]) -> (f32, f32) {
        let n = data.len() as f32;
        let mean = data.iter().sum::<f32>() / n;
        let var = data.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / n;
        (mean, var)
    }

    #[test]
    fn welford_matches_naive_on_small_data() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        let mut w = WelfordState::new();
        for &x in &data {
            w.update(x);
        }
        let (naive_mean, naive_var) = naive_mean_var(&data);
        // Welford is online and uses count-based, slightly different bias but very close.
        // Use tight tolerance for this deterministic path.
        assert!((w.mean - naive_mean).abs() < 1e-6);
        // Note: population variance vs sample; our impl is population-style.
        assert!((w.variance() - naive_var).abs() < 1e-5);
    }

    #[test]
    fn welford_std_is_stable_and_positive() {
        let data = [0.1f32, -0.2, 0.3];
        let mut w = WelfordState::new();
        for &x in &data {
            w.update(x);
        }
        let s = w.std(1e-5);
        assert!(s > 0.0 && s.is_finite());
    }
}
