//! NV-style FP8 E4M3 (deterministic encode/decode for shadow path).

/// Supported FP8 layouts for shadow storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fp8Format {
    E4M3,
}

const EXP_BIAS: i32 = 7;
const MAX_FINITE: f32 = 448.0;

fn frexp_f32(x: f32) -> (f32, i32) {
    if x == 0.0 {
        return (0.0, 0);
    }
    let bits = x.to_bits();
    let exp = ((bits >> 23) & 0xff) as i32 - 127;
    let mant = f32::from_bits((bits & 0x807f_ffff) | 0x3f00_0000);
    (mant, exp)
}

/// Encode f32 → u8 E4M3 (deterministic; NaN/Inf → max finite with sign).
pub fn encode_e4m3(x: f32) -> u8 {
    if x == 0.0 {
        return 0;
    }
    let sign_bit = if x.is_sign_negative() { 0x80u8 } else { 0u8 };
    let mut ax = x.abs();
    if !ax.is_finite() {
        return sign_bit | 0x7f;
    }
    if ax > MAX_FINITE {
        ax = MAX_FINITE;
    }

    if ax < 2.0f32.powi(-6) {
        let mant_bits = (ax / 2.0f32.powi(-6) * 8.0).round().clamp(0.0, 7.0) as u8;
        return sign_bit | mant_bits;
    }

    let (m, mut e) = frexp_f32(ax);
    let mant = m * 2.0 - 1.0;
    e = e.clamp(-6, 8);
    let mut mant_bits = (mant * 8.0).round() as i32;
    if mant_bits >= 8 {
        mant_bits = 0;
        e += 1;
    }
    let exp_bits = (e + EXP_BIAS).clamp(1, 15) as u8;
    sign_bit | (exp_bits << 3) | (mant_bits as u8)
}

/// Decode u8 E4M3 → f32 (fixed algorithm).
pub fn decode_e4m3(bits: u8) -> f32 {
    if bits == 0 {
        return 0.0;
    }
    let sign = if bits & 0x80 != 0 { -1.0 } else { 1.0 };
    let exp_bits = (bits >> 3) & 0x0f;
    let mant_bits = bits & 0x07;
    if exp_bits == 0 {
        return sign * (mant_bits as f32) * 2.0f32.powi(-6) / 8.0;
    }
    let exp = exp_bits as i32 - EXP_BIAS;
    let mant = 1.0 + (mant_bits as f32) / 8.0;
    sign * mant * 2.0f32.powi(exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_representable_grid() {
        for &x in &[0.0f32, 0.5, 1.0, 2.0, 4.0, -2.0, 8.0] {
            let b = encode_e4m3(x);
            let y = decode_e4m3(b);
            assert!(
                (x - y).abs() < 1e-6 || x.to_bits() == y.to_bits(),
                "x={x} y={y} bits={b}"
            );
        }
    }
}