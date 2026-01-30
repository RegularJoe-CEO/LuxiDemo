# LuxiEdge for ML Inference

## The Problem

Your model works in PyTorch. Export to ONNX, results drift. Deploy to TensorRT, more drift. Debugging inference errors across frameworks wastes months.

## How LuxiEdge Solves This

Run activation functions and attention math with bit-exact reproducibility. Same input, same hash, regardless of deployment target.

## Activation Functions

| Function | Use Case |
|----------|----------|
| x * normcdf(x * 1.702) | GELU (GPT, BERT, Grok) |
| x / (1 + exp(-x)) | SiLU / Swish |
| ln(1 + exp(x)) | Softplus |
| x / sqrt(x^2 + epsilon) | RMS Norm |
| -ln(x + 1e-7) | Cross-Entropy Component |
| (exp(x) - exp(-x)) / (exp(x) + exp(-x)) | Tanh |

## Example

```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"x * normcdf(x * 1.702)","values":[-2,-1,0,1,2],"precision":"f32"}'
```

## Framework-Agnostic Debugging

1. Extract intermediate activations from PyTorch
2. Run same values through LuxiEdge
3. Compare SHA256 hashes
4. Mismatch = framework bug, not your code

## Contact

e@ewaller.com
