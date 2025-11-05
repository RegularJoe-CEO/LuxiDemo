# Finding the Current Benchmark Data

If you still see the October 2024 snapshot after recent updates, walk through the
checks below to make sure you are viewing the refreshed January 2025 results.

## 1. Verify on GitHub

1. Navigate to the repository home page and select the **main** branch from the
   branch picker in the upper-left corner.
2. Click `BENCHMARK_DATA.md` in the file browser.
3. Use the **History** button (top right) and confirm that the most recent entry is
   the commit titled **"Clarify benchmark freshness guidance"** dated **2025‑01‑18**.
   - If you do not see that commit, switch branches or forks until you are on the
     canonical `main` branch.
4. Return to the file view, click the **Raw** button, and use your browser refresh
   shortcut (`Cmd/Ctrl + Shift + R`) to bypass cached copies. You should now see the
   "Last updated: 2025-01-18" banner near the top of the document.

## 2. Verify in a Local Clone

1. Run `./tools/verify_benchmark_freshness.sh` from the repo root to print the
   active branch, the last commit touching `BENCHMARK_DATA.md`, and any duplicate
   benchmark summaries that need cleanup.
2. If the script reports an unexpected branch or commit date, switch to `main`
   (`git checkout main`) and pull the latest code with `git pull --ff-only origin main`.
3. Force-replace any stale working tree copy with:

   ```bash
   rm -f BENCHMARK_DATA.md
   git checkout -- BENCHMARK_DATA.md
   ```

4. List the benchmark hub to make sure the supporting documents exist:

   ```bash
   ls docs/benchmarks
   ls docs/benchmarks/data_exports
   ```

   You should see `COMPARATIVE_ANALYSIS.md`, `FINDING_DATA.md`, `README.md`, and any
   Criterion baselines you have exported.

## 3. Removing Legacy Artifacts

`SYNCING_MAIN.md` includes a dedicated cleanup section for wiping the retired
October files. The short version:

```bash
git clean -xdf docs/benchmarks/archive 2>/dev/null || true
rm -f BENCHMARK_DATA_old.md BENCHMARK_DATA_OCT*.md 2>/dev/null || true
```

Only run these commands if you do not need the deprecated files. They do not touch
the current January 2025 documents.

## 4. Need More Help?

* Re-run `cargo bench --bench edge_suite` and `cargo bench --bench my_benchmark` to
  regenerate the latest figures locally.
* Open an issue with screenshots of the GitHub history or terminal output if the
  commit still does not appear after following the steps above.

With these checks you should always land on the up-to-date benchmark suite without
running into the archived October revision.
