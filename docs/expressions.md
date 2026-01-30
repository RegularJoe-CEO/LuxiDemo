# LuxiEdge Expression Reference

## Supported Functions

| Function | Description | Valid Input Range |
|----------|-------------|-------------------|
| sin(x) | Sine | Any |
| cos(x) | Cosine | Any |
| tan(x) | Tangent | x ≠ π/2 + nπ |
| exp(x) | e^x | x < 88 (f32) |
| ln(x) | Natural log | x > 0 |
| sqrt(x) | Square root | x ≥ 0 |
| erf(x) | Error function | Any |
| gamma(x) | Gamma function | x > 0, x ≠ negative integers |
| normcdf(x) | Cumulative normal | Any |
| normpdf(x) | Normal density | Any |
| abs(x) | Absolute value | Any |
| neg(x) | Negation | Any |
| asin(x) | Arcsine | -1 ≤ x ≤ 1 |
| acos(x) | Arccosine | -1 ≤ x ≤ 1 |
| atan(x) | Arctangent | Any |

## Operators

| Operator | Example |
|----------|---------|
| + | x + 1 |
| - | x - 1 |
| * | x * 2 |
| / | x / 2 |
| ^ | x^2 |
| () | Grouping |

Precedence: ^ > * / > + -

Parentheses override precedence.

## 800+ Expressions

15 functions + 6 operators = unlimited combinations. Chain them:

- `x * normcdf(x * 1.702)` → GELU activation
- `sqrt((2/x) - (1/a))` → Vis-Viva orbital velocity
- `(exp(x) - exp(-x)) / (exp(x) + exp(-x))` → Tanh
- `exp(-x^2) * cos(x * 10)` → Gaussian envelope
- `-0.75 * ln(1 - (4 * x / 3))` → Jukes-Cantor distance

## Chaining Expressions

Nest functions and combine with operators:

```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"exp(sin(x))*cos(x^2)","values":[0.1,0.2,0.3],"precision":"f32"}'
```

## Example Combinations

### Finance: Black-Scholes Components
normcdf((ln(x) + 0.5) / sqrt(x))




### Signal Processing: Damped Oscillation
exp(-x) * sin(x * 6.28)




### Physics: Gaussian Envelope
exp(-x^2) * cos(x * 10)




### Statistics: Log-Normal Density
normpdf(ln(x)) / x




### Numerical Methods: Sigmoid Approximation
0.5 * (1 + erf(x / sqrt(2)))




## Batch Processing

Pass thousands of values in one call:

```python
import requests

values = [0.001 * i for i in range(10000)]

r = requests.post(
    "http://localhost:10000/evaluate",
    json={
        "expr": "sin(x) * exp(-x^2)",
        "values": values,
        "precision": "f32"
    }
)

print(r.json()["sha256"])
Precision
Option	Bits	Use Case
f32	32	Standard, faster
f64	64	Higher precision
Both produce deterministic SHA256 hashes.

Genomics & Bioinformatics
Sequence Alignment Scoring
Log-odds ratio for base match probability:



ln(x / (1 - x))
Mutation Probability (Poisson)
Expected mutations over time t with rate λ:



exp(-x) * (x^2) / gamma(3)
Population Genetics: Hardy-Weinberg
Heterozygote frequency given allele frequency x:



2 * x * (1 - x)
PHRED Quality Score Conversion
Error probability from quality score:



exp(-x * ln(10) / 10)
GC Content Normalization
Z-score for GC bias correction:



(x - 0.5) / sqrt(0.25 / 100)
Phylogenetic Distance (Jukes-Cantor)
Evolutionary distance from sequence divergence:



-0.75 * ln(1 - (4 * x / 3))
Contact
e@ewaller.com
