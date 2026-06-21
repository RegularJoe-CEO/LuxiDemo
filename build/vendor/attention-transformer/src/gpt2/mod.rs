//! GPT-2 (124M) model runner on top of the existing WNSM/GAE attention engine.
//!
//! This module is **only compiled** when the `gpt2` Cargo feature is enabled.
//! It provides:
//! - `loader`: memory-mapped safetensors loading + tensor conversion (handles Conv1D transpose).
//! - `tokenizer`: HF tokenizers crate wrapper for BPE encode/decode.
//! - `model`: pure-Rust GPT-2 forward pass that re-uses engine primitives
//!   (layernorm, waller_operator for causal attention, GELU, etc.) while
//!   faithfully implementing the pre-LN GPT-2 architecture.
//!
//! Hard constraints observed:
//! - CPU only. No GPU paths are referenced or built.
//! - Existing public API and core math in the engine are untouched.
//! - All new code lives under src/gpt2/ and is feature-gated.
//! - Determinism and receipts are preserved for the new forward pass.

pub mod draft_head;
pub mod loader;
pub mod tokenizer;
pub mod model;

pub use loader::{find_gpt2_snapshot_dir, load_gpt2_tensors, Gpt2Tensors};
pub use tokenizer::Gpt2Tokenizer;
pub use draft_head::DraftHead;
pub use model::{Gpt2Config, Gpt2Model};