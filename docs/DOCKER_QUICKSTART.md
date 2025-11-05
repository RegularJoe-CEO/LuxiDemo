# Luxi Edge — Docker Quickstart

Pull and run:
  docker run --rm -p 8080:8080 ghcr.io/regularjoe-ceo/luxi-edge:latest

Smoke test:
  curl -s http://127.0.0.1:8080/health | python3 -m json.tool
  curl -s -H 'content-type: application/json' \
    -d '{"expr":"x*x + 2*x + 1","x":[0,1,2,3]}' \
    http://127.0.0.1:8080/evaluate | python3 -m json.tool

Notes:
- Default port: 8080
- Precision: default f64; optional "precision" field is accepted now; compute path maps to f64 for the moment.
