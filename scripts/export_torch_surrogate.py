#!/usr/bin/env python3
"""
Export a PyTorch neural surrogate model to ONNX format for Lambert TOF prediction.

This script demonstrates how to train and export a simple neural network that
can predict time-of-flight (TOF) from orbit parameters. The exported ONNX model
can be loaded by the Rust neural_surrogate module.

Requirements:
    pip install torch numpy

Usage:
    python3 scripts/export_torch_surrogate.py --output model.onnx --samples 10000

The model architecture:
    Input: [a, r1, r2, c, s, mu, n_rev] (7 features)
    Hidden: 2 layers with 64 neurons each
    Output: [tof, confidence] (2 values)
"""

import argparse
import math
import numpy as np
import torch
import torch.nn as nn


def lambert_tof(a, r1, r2, c, s, mu, n_rev=0):
    """
    Calculate Lambert TOF using the same formula as the Rust implementation.
    
    This is the "ground truth" physics model that the neural network learns to approximate.
    """
    if a <= s / 2.0:
        return float('nan')
    
    # Calculate alpha and beta
    alpha_sin = math.sqrt(s / (2.0 * a))
    beta_sin = math.sqrt((s - c) / (2.0 * a))
    
    alpha = 2.0 * math.asin(alpha_sin)
    beta = 2.0 * math.asin(beta_sin)
    
    # Time of flight formula
    tof_base = math.sqrt(a**3 / mu) * (alpha - math.sin(alpha) - (beta - math.sin(beta)))
    tof_multi = tof_base + 2.0 * math.pi * n_rev * math.sqrt(a**3 / mu)
    
    return tof_multi


def generate_training_data(n_samples=10000, seed=42):
    """
    Generate synthetic training data by sampling orbit parameters
    and computing exact TOF using physics simulation.
    """
    np.random.seed(seed)
    
    # Typical orbit parameter ranges (based on LEO to GEO transfers)
    a_min, a_max = 6500.0, 30000.0  # Semi-major axis (km)
    r1_min, r1_max = 6578.0, 42164.0  # Departure radius (km)
    r2_min, r2_max = 6578.0, 42164.0  # Arrival radius (km)
    mu = 398600.0  # Earth gravitational parameter (km³/s²)
    
    X = []
    y = []
    
    for _ in range(n_samples):
        a = np.random.uniform(a_min, a_max)
        r1 = np.random.uniform(r1_min, r1_max)
        r2 = np.random.uniform(r2_min, r2_max)
        
        # Calculate derived parameters
        c = abs(r2 - r1) + np.random.uniform(0, min(r1, r2) * 0.5)
        s = (r1 + r2 + c) / 2.0
        
        # Only include valid samples
        if a > s / 2.0:
            n_rev = np.random.randint(0, 3)  # 0, 1, or 2 revolutions
            
            try:
                tof = lambert_tof(a, r1, r2, c, s, mu, n_rev)
                if not math.isnan(tof) and tof > 0:
                    X.append([a, r1, r2, c, s, mu, n_rev])
                    y.append([tof, 1.0])  # confidence = 1.0 for training data
            except:
                continue
    
    return np.array(X, dtype=np.float32), np.array(y, dtype=np.float32)


class LambertSurrogate(nn.Module):
    """
    Neural network surrogate for Lambert TOF prediction.
    
    Architecture:
    - Input layer: 7 features (a, r1, r2, c, s, mu, n_rev)
    - Hidden layers: 2 × 64 neurons with ReLU activation
    - Output layer: 2 outputs (tof, confidence)
    """
    
    def __init__(self, input_dim=7, hidden_dim=64, output_dim=2):
        super(LambertSurrogate, self).__init__()
        
        self.network = nn.Sequential(
            nn.Linear(input_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, output_dim)
        )
        
        # Output activation: sigmoid for confidence (0-1 range)
        self.sigmoid = nn.Sigmoid()
    
    def forward(self, x):
        out = self.network(x)
        # Apply sigmoid only to confidence output
        out[:, 1] = self.sigmoid(out[:, 1])
        return out


def train_surrogate(X_train, y_train, epochs=100, lr=0.001):
    """Train the neural surrogate model."""
    model = LambertSurrogate()
    criterion = nn.MSELoss()
    optimizer = torch.optim.Adam(model.parameters(), lr=lr)
    
    X_tensor = torch.from_numpy(X_train)
    y_tensor = torch.from_numpy(y_train)
    
    print(f"Training on {len(X_train)} samples for {epochs} epochs...")
    
    for epoch in range(epochs):
        # Forward pass
        outputs = model(X_tensor)
        loss = criterion(outputs, y_tensor)
        
        # Backward pass
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        
        if (epoch + 1) % 10 == 0:
            print(f"Epoch [{epoch+1}/{epochs}], Loss: {loss.item():.4f}")
    
    return model


def export_to_onnx(model, output_path="lambert_surrogate.onnx"):
    """Export the trained model to ONNX format."""
    model.eval()
    
    # Create dummy input (batch_size=1, input_dim=7)
    dummy_input = torch.randn(1, 7)
    
    torch.onnx.export(
        model,
        dummy_input,
        output_path,
        export_params=True,
        opset_version=14,
        do_constant_folding=True,
        input_names=['input'],
        output_names=['output'],
        dynamic_axes={
            'input': {0: 'batch_size'},
            'output': {0: 'batch_size'}
        }
    )
    
    print(f"✓ Model exported to {output_path}")
    print(f"  Input shape: (batch_size, 7)")
    print(f"  Output shape: (batch_size, 2) - [tof, confidence]")


def main():
    parser = argparse.ArgumentParser(description="Export PyTorch Lambert surrogate to ONNX")
    parser.add_argument("--output", default="lambert_surrogate.onnx", help="Output ONNX file path")
    parser.add_argument("--samples", type=int, default=10000, help="Number of training samples")
    parser.add_argument("--epochs", type=int, default=100, help="Training epochs")
    parser.add_argument("--lr", type=float, default=0.001, help="Learning rate")
    args = parser.parse_args()
    
    print("=== Lambert TOF Neural Surrogate Export ===\n")
    
    # Generate training data
    print(f"Generating {args.samples} training samples...")
    X_train, y_train = generate_training_data(args.samples)
    print(f"✓ Generated {len(X_train)} valid samples\n")
    
    # Train model
    model = train_surrogate(X_train, y_train, epochs=args.epochs, lr=args.lr)
    print()
    
    # Export to ONNX
    export_to_onnx(model, args.output)
    print()
    
    # Test the model
    print("Testing model predictions:")
    model.eval()
    test_input = torch.tensor([[7000.0, 6980.0, 10520.0, 6655.0, 12078.0, 398600.0, 0.0]])
    with torch.no_grad():
        prediction = model(test_input)
        pred_tof = prediction[0, 0].item()
        pred_conf = prediction[0, 1].item()
    
    # Calculate actual TOF
    actual_tof = lambert_tof(7000.0, 6980.0, 10520.0, 6655.0, 12078.0, 398600.0, 0)
    error = abs(pred_tof - actual_tof)
    
    print(f"  Input: a=7000.0, r1=6980.0, r2=10520.0")
    print(f"  Predicted TOF: {pred_tof:.2f}s (confidence: {pred_conf:.3f})")
    print(f"  Actual TOF: {actual_tof:.2f}s")
    print(f"  Error: {error:.2f}s ({error/actual_tof*100:.2f}%)")
    print()
    
    print("=== Usage in Rust ===")
    print(f"// Load the model")
    print(f"let config = SurrogateConfig {{")
    print(f"    model_path: Some(\"{args.output}\".to_string()),")
    print(f"    confidence_threshold: 0.95,")
    print(f"    ..Default::default()")
    print(f"}};")
    print(f"let surrogate = NeuralSurrogate::from_onnx(\"{args.output}\", config)?;")
    print()
    print(f"// Use in hybrid Monte Carlo")
    print(f"let (samples, stats) = hybrid_monte_carlo_tof(")
    print(f"    6066.0, 10.0, 6980.0, 10520.0, 6655.0, 12078.0, 398600.0,")
    print(f"    0, 1000, Some(&surrogate)")
    print(f")?;")


if __name__ == "__main__":
    main()
