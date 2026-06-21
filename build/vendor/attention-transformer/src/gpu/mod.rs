// src/gpu/mod.rs
// Optional GPU acceleration backends for attention-transformer.
//
// The primary strength of this technology is the **pure Rust reference**,
// which runs efficiently on edge devices, laptops, ARM, and any constrained
// hardware (excellent calcs per joule due to algorithmic data-movement reductions).
//
// CUDA (and future Metal / WebGPU) paths are optional accelerators for when
// high-end hardware is available. They must match the Rust reference exactly
// on outputs and cryptographic receipts.
//
// Current state:
// - CUDA: Real implementation + usable dispatch (behind "cuda" feature)
// - Metal & WebGPU: Directories exist with initial stubs (future work)

#[cfg(feature = "cuda")]
pub mod cuda;

// `cuda` module already exported via `pub mod cuda;` above.
