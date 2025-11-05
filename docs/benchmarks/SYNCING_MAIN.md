# Keeping `main` Up to Date for Benchmark Documents

Use this playbook when GitHub or a local checkout still shows the October benchmark snapshot. The steps align `main` with the latest January 2025 documentation and remove stale artifacts.

## 1. Confirm You Are on `main`

```bash
git status -sb
```

- If the output starts with `## main`, you are already on the main branch.
- If it shows another branch (for example `## work`), switch with:
  ```bash
  git checkout main
  ```

## 2. Fast-Forward to the Latest Remote Commit

```bash
git pull --ff-only origin main
```

This command refuses to create merge commits so `main` exactly matches the remote history. If it fails, run `git fetch origin main` followed by `git reset --hard origin/main` to realign.

## 3. Remove Cached or Legacy Copies

Clean out tracked and untracked files that belong to the October snapshot:

```bash
git clean -xdf docs/benchmarks/archive 2>/dev/null || true
rm -f BENCHMARK_DATA_old.md BENCHMARK_DATA_OCT*.md 2>/dev/null || true
```

These files are safe to delete—they were part of the deprecated benchmark layout.

## 4. Verify the Freshness Script

Run the helper script from the repository root:

```bash
./tools/verify_benchmark_freshness.sh
```

You should see:
- `Current branch: main`
- The latest commit hash from `origin/main`
- The January 2025 `git log` entry for `BENCHMARK_DATA.md`

## 5. Double-Check on GitHub

1. Navigate to the repository in your browser.
2. Ensure the branch selector in the top-left corner reads **main**.
3. Open `BENCHMARK_DATA.md`.
4. Click **History** and confirm the latest entry references January 2025.
5. Hard refresh the page (`Shift+Reload`) if the page still shows the October version.

Following this checklist guarantees that both your local environment and the GitHub UI surface the refreshed benchmark documents.
