//! Configuration for the attention transformer.

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub hidden_dim: usize,
    pub num_heads: usize,
    pub head_dim: usize,
    pub mlp_dim: usize,
    pub ln_eps: f32,
    pub max_seq_len: usize,
}

impl Config {
    pub fn new(hidden_dim: usize, num_heads: usize, mlp_dim: usize, max_seq_len: usize) -> Self {
        assert!(
            hidden_dim % num_heads == 0,
            "hidden_dim must be divisible by num_heads"
        );
        Self {
            hidden_dim,
            num_heads,
            head_dim: hidden_dim / num_heads,
            mlp_dim,
            ln_eps: 1e-5,
            max_seq_len,
        }
    }

    /// Common small configuration for testing and demos.
    pub fn small() -> Self {
        Self::new(64, 4, 256, 128)
    }

    /// GPT-2 Small style (for loading real weights later).
    pub fn gpt2_small() -> Self {
        Self::new(768, 12, 3072, 1024)
    }
}
