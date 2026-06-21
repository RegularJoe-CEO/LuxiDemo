//! Trained linear draft head for speculative decoding.
//!
//! Architecture: simple linear layer  n_embd → vocab  (or top-K distilled).
//! The head is deliberately tiny — one matrix + bias — so the "cost" of drafting
//! is a single small matmul on hidden states the verifier already computed.

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Clone)]
pub struct DraftHead {
    pub n_embd: usize,
    pub vocab: usize,
    pub weight: Vec<f32>, // [vocab, n_embd] row-major (logits = hidden @ W^T + b)
    pub bias: Vec<f32>,   // [vocab]
}

impl DraftHead {
    pub fn new(n_embd: usize, vocab: usize) -> Self {
        Self {
            n_embd,
            vocab,
            weight: vec![0.0; vocab * n_embd],
            bias: vec![0.0; vocab],
        }
    }

    /// Load from a simple binary format: u32 n_embd, u32 vocab, then weight bytes, then bias bytes.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let mut f = File::open(path.as_ref())
            .map_err(|e| format!("Failed to open draft head: {}", e))?;

        let mut header = [0u8; 8];
        f.read_exact(&mut header).map_err(|e| format!("header read failed: {}", e))?;

        let n_embd = u32::from_le_bytes(header[0..4].try_into().unwrap()) as usize;
        let vocab = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;

        let mut weight = vec![0.0f32; vocab * n_embd];
        let weight_bytes = unsafe {
            std::slice::from_raw_parts_mut(weight.as_mut_ptr() as *mut u8, weight.len() * 4)
        };
        f.read_exact(weight_bytes).map_err(|e| format!("weight read failed: {}", e))?;

        let mut bias = vec![0.0f32; vocab];
        let bias_bytes = unsafe {
            std::slice::from_raw_parts_mut(bias.as_mut_ptr() as *mut u8, bias.len() * 4)
        };
        f.read_exact(bias_bytes).map_err(|e| format!("bias read failed: {}", e))?;

        Ok(Self { n_embd, vocab, weight, bias })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let mut f = File::create(path.as_ref())
            .map_err(|e| format!("Failed to create draft head file: {}", e))?;

        f.write_all(&(self.n_embd as u32).to_le_bytes()).map_err(|e| e.to_string())?;
        f.write_all(&(self.vocab as u32).to_le_bytes()).map_err(|e| e.to_string())?;

        let weight_bytes = unsafe {
            std::slice::from_raw_parts(self.weight.as_ptr() as *const u8, self.weight.len() * 4)
        };
        f.write_all(weight_bytes).map_err(|e| e.to_string())?;

        let bias_bytes = unsafe {
            std::slice::from_raw_parts(self.bias.as_ptr() as *const u8, self.bias.len() * 4)
        };
        f.write_all(bias_bytes).map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Apply the head to a single hidden vector. Returns logits.
    #[inline]
    pub fn apply(&self, hidden: &[f32]) -> Vec<f32> {
        let mut logits = self.bias.clone();
        for v in 0..self.vocab {
            let mut s = 0.0f32;
            let row = &self.weight[v * self.n_embd..(v + 1) * self.n_embd];
            for d in 0..self.n_embd {
                s += hidden[d] * row[d];
            }
            logits[v] += s;
        }
        logits
    }
}