// Legacy tests from the original repo initialization.
// They continue to exercise the public API surface.
// The first test now runs through the real deterministic WNSM code path.

use attention_transformer::{transformer_forward, Tensor};

#[test]
fn preserves_tensor_bits_for_deterministic_input() {
    let input = Tensor::new(vec![0.25, -1.0, 3.5, 7.0], vec![2, 2]);
    let output = transformer_forward(input.clone());
    assert_eq!(output, input);
}

#[test]
#[should_panic(expected = "tensor shape does not match data length")]
fn rejects_invalid_tensor_shape() {
    let _ = Tensor::new(vec![1.0, 2.0, 3.0], vec![2, 2]);
}
