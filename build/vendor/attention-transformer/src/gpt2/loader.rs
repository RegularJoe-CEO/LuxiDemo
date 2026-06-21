//! GPT-2 weight loader using memory-mapped safetensors.
//!
//! Resolves the snapshot directory at runtime by globbing
//! ~/.cache/huggingface/hub/models--gpt2/snapshots/* (there is one).
//! Handles the GPT-2 Conv1D weight layout gotcha (stored transposed relative
//! to a standard Linear, i.e. the HF implementation does x @ W not x @ W^T
//! for the c_* layers).
//!
//! All tensors are converted to Vec<f32> (supports stored f32 and f16).

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use memmap2::Mmap;
use safetensors::{Dtype, SafeTensors};

/// GPT-2 tensor storage after loading and dtype conversion.
#[derive(Clone)]
pub struct Gpt2Tensors {
    pub tensors: HashMap<String, Vec<f32>>,
    pub shapes: HashMap<String, Vec<usize>>,
}

impl Gpt2Tensors {
    pub fn get(&self, name: &str) -> Option<(&[f32], &[usize])> {
        let data = self.tensors.get(name)?;
        let shape = self.shapes.get(name)?;
        Some((data, shape))
    }
}

/// Resolves the GPT-2 snapshot directory by globbing the snapshots folder.
/// Returns the path to the (only) snapshot directory containing the files.
pub fn find_gpt2_snapshot_dir() -> Result<PathBuf, String> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Could not determine home directory (HOME/USERPROFILE)".to_string())?;

    let snapshots_base = Path::new(&home)
        .join(".cache/huggingface/hub/models--gpt2/snapshots");

    if !snapshots_base.exists() {
        return Err(format!(
            "GPT-2 snapshot directory not found at {:?}. \
             Expected the model to be pre-downloaded via huggingface_hub.",
            snapshots_base
        ));
    }

    let mut snapshots: Vec<PathBuf> = std::fs::read_dir(&snapshots_base)
        .map_err(|e| format!("Failed to read snapshots dir: {}", e))?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();

    if snapshots.is_empty() {
        return Err(format!(
            "No snapshot directories found under {:?}",
            snapshots_base
        ));
    }

    // There is one (as per spec). Take the first (or only) one.
    snapshots.sort();
    let chosen = snapshots.remove(0);

    // Verify key files exist (via symlinks is fine)
    let safetensors = chosen.join("model.safetensors");
    let config = chosen.join("config.json");
    if !safetensors.exists() || !config.exists() {
        return Err(format!(
            "Required files missing in snapshot {:?} (model.safetensors + config.json)",
            chosen
        ));
    }

    Ok(chosen)
}

/// Loads all tensors from model.safetensors (mmap) and converts to f32.
/// Applies the GPT-2 Conv1D transpose correction for c_attn/c_proj/c_fc layers.
pub fn load_gpt2_tensors(snapshot_dir: &Path) -> Result<Gpt2Tensors, String> {
    let safetensors_path = snapshot_dir.join("model.safetensors");

    let file = File::open(&safetensors_path)
        .map_err(|e| format!("Failed to open model.safetensors: {}", e))?;

    // SAFETY: The file is not mutated while the mmap lives, and we only read.
    let mmap = unsafe {
        Mmap::map(&file).map_err(|e| format!("Failed to mmap safetensors: {}", e))?
    };

    let safetensors = SafeTensors::deserialize(&mmap)
        .map_err(|e| format!("Failed to parse safetensors header: {}", e))?;

    let mut tensors = HashMap::new();
    let mut shapes = HashMap::new();

    for name in safetensors.names() {
        let tensor_view: safetensors::tensor::TensorView = safetensors
            .tensor(name)
            .map_err(|e| format!("Failed to get tensor '{}': {}", name, e))?;

        let shape: Vec<usize> = tensor_view.shape().to_vec();
        let dtype = tensor_view.dtype();
        let raw_data = tensor_view.data();

        let mut data: Vec<f32> = match dtype {
            Dtype::F32 => {
                if raw_data.len() % 4 != 0 {
                    return Err(format!("Invalid f32 data length for {}", name));
                }
                raw_data
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                    .collect()
            }
            Dtype::F16 => {
                if raw_data.len() % 2 != 0 {
                    return Err(format!("Invalid f16 data length for {}", name));
                }
                raw_data
                    .chunks_exact(2)
                    .map(|b| {
                        let bits = u16::from_le_bytes([b[0], b[1]]);
                        half_to_f32(bits)
                    })
                    .collect()
            }
            Dtype::BF16 => {
                if raw_data.len() % 2 != 0 {
                    return Err(format!("Invalid bf16 data length for {}", name));
                }
                raw_data
                    .chunks_exact(2)
                    .map(|b| {
                        let bits = u16::from_le_bytes([b[0], b[1]]);
                        bf16_to_f32(bits)
                    })
                    .collect()
            }
            other => {
                return Err(format!(
                    "Unsupported dtype {:?} for tensor '{}'. Convert model to f32/f16.",
                    other, name
                ))
            }
        };

        // GPT-2 Conv1D transpose correction (critical for coherent output)
        if (name.contains("c_attn.weight") || name.contains("c_proj.weight") || name.contains("c_fc.weight"))
            && shape.len() == 2
        {
            let (rows, cols) = (shape[0], shape[1]);
            data = transpose_2d(&data, rows, cols);
        }

        tensors.insert(name.to_string(), data);
        shapes.insert(name.to_string(), shape);
    }

    Ok(Gpt2Tensors { tensors, shapes })
}

fn transpose_2d(data: &[f32], rows: usize, cols: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; rows * cols];
    for r in 0..rows {
        for c in 0..cols {
            out[c * rows + r] = data[r * cols + c];
        }
    }
    out
}

/// Minimal f16 (IEEE 754 half) to f32 conversion (no_std friendly).
#[inline]
fn half_to_f32(h: u16) -> f32 {
    let sign = (h >> 15) & 0x1;
    let exp = (h >> 10) & 0x1f;
    let mant = h & 0x3ff;

    if exp == 0 {
        if mant == 0 {
            return if sign == 1 { -0.0 } else { 0.0 };
        }
        // subnormal
        let f = (mant as f32) / 1024.0 * 2f32.powi(-14);
        return if sign == 1 { -f } else { f };
    }
    if exp == 0x1f {
        if mant == 0 {
            return if sign == 1 { f32::NEG_INFINITY } else { f32::INFINITY };
        }
        return f32::NAN;
    }

    let f = (1.0 + (mant as f32) / 1024.0) * 2f32.powi((exp as i32) - 15);
    if sign == 1 { -f } else { f }
}

/// Minimal bf16 to f32 (treats as f32 with exponent bias adjustment).
#[inline]
fn bf16_to_f32(bf: u16) -> f32 {
    // bf16 has same exponent as f32 but 7 mantissa bits.
    let bits = (bf as u32) << 16;
    f32::from_bits(bits)
}