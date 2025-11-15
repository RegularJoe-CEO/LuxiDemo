# eRock Demo Instructions (Local Engine, Single Machine)

This file is the **start here** guide for running the eRock engine on your own machine.

It assumes:

- You are on a Mac with a local clone of this repo at `~/eRock`
- Rust toolchain is installed (`rustup` + `cargo`)
- You want a simple, repeatable way to:
  - Start the engine
  - Run a quick smoke test
  - Run the CPU benchmark demo

For deeper detail, see `RUN_ENGINE.md` in this same directory.

---

## 1. Confirm you are in the right place

In Terminal:

```bash
cd ~/eRock
pwd
ls
```

You should see something like:

- `Cargo.toml`
- `src/`
- `RUN_ENGINE.md`
- `DEMO_INSTRUCTIONS.md` (this file)
- `scripts/`

If that’s not what you see, stop and correct the path before proceeding.

---

## 2. Build the project (one-time per code change)

From the repo root:

```bash
cd ~/eRock
cargo build --release
```

- This compiles the engine in release mode.
- You only need to repeat this after code changes or dependency updates.

If this fails, do **not** guess; fix the build first before trying to run anything.

---

## 3. Run the engine (follow the authoritative details)

The **authoritative, low-level run instructions** live in `RUN_ENGINE.md`.

From repo root:

```bash
cd ~/eRock
open RUN_ENGINE.md
```

Follow the “Start the server” / “Run the engine” section in that file exactly.

> **Important:**  
> `RUN_ENGINE.md` is the source of truth for:
> - The exact `cargo run` command
> - The correct binary name
> - The correct port and HTTP endpoints  
> This `DEMO_INSTRUCTIONS.md` is the quick index that always points you there.

Once the engine is running in one Terminal window, leave it running.

---

## 4. Smoke-test the running engine

After the server is started (per `RUN_ENGINE.md`), use the smoke test described there.

Typical pattern (for reference only; the exact endpoint is defined in `RUN_ENGINE.md`):

- A `/health` or `/ping` endpoint to confirm the process is alive
- An `/evaluate` endpoint to send a small math expression and get a numeric result

Run the exact test commands from `RUN_ENGINE.md`.  
If those succeed, the engine is ready for demo.

---

## 5. Run the CPU benchmark demo (automated script)

This uses the preconfigured CPU benchmark script in `scripts/`.

From a **new Terminal window** (with the server **stopped** unless `RUN_ENGINE.md` says otherwise):

```bash
cd ~/eRock
./scripts/run_cpu_suite.sh
```

What this script does:

- Runs `cargo build --release` (if needed)
- Runs a series of `cargo bench` benchmarks (e.g. evaluate_10k, evaluate_100k, bisect_root, simd_inplace_100k)
- Captures logs into `benchmark_logs/` for later review

Notes:

- Expect this to be **CPU-heavy** and take on the order of **40–60 minutes** on a Mac, depending on configuration.
- Criterion may print warnings like “Unable to complete 100 samples” when the tests are very heavy; that is expected in this setup.

When it finishes, you should see log files under:

```bash
cd ~/eRock
ls benchmark_logs
```

---

## 6. Stop the engine cleanly

If you have the engine running in a Terminal window (from `RUN_ENGINE.md`):

- Focus that window
- Press `Ctrl+C` to stop the server

Confirm in `top` or `ps` that the server process is no longer running if you’re unsure.

---

## 7. Optional: Desktop copy of these directions

If you want a desktop copy named “directions to run this engine.md”:

From repo root:

```bash
cd ~/eRock
cp DEMO_INSTRUCTIONS.md ~/Desktop/"directions to run this engine.md"
```

Note:

- `~` is **outside** the quotes so it expands to your home directory.
- This just copies the file; it does **not** run any code or tests.

---

## 8. What to trust

- **Trust:** `DEMO_INSTRUCTIONS.md` (this file) + `RUN_ENGINE.md` at repo root.
- Ignore the branch jungle for demo purposes. As long as you are in `~/eRock` and these two files exist at the top level, you have what you need to:
  - Build
  - Run
  - Smoke-test
  - Benchmark
the eRock engine on your Mac.
