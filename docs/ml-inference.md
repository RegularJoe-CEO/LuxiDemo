# LuxiEdge for ML Inference

> Market hypothesis only. This page does not claim a shipping product, customer deployment, certification, or production use. Public claims remain limited to the named demos and evidence.

## The Problem

Your model works in PyTorch. Export to ONNX, results drift. Deploy to TensorRT, more drift. Debugging inference errors across frameworks wastes months.

## What the demo explores

Activation and attention calls return a SHA256 of the bytes computed on that run. Re-running the same binary is stable. Cross-target identity was not measured for these kernels, and GPU was not measured.

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
