# Documentation Cleanup Prompt Template

**Purpose:** Use this prompt to ensure AI assistants perform COMPLETE repository-wide documentation updates without manual oversight.

---

## The Complete Cleanup Prompt

```
I need a COMPREHENSIVE documentation update across the ENTIRE repository. 

CRITICAL REQUIREMENTS:
1. Check EVERY markdown file in the repository (root level + all subdirectories)
2. Update ALL files that should contain [SPECIFIC DATA/CHANGE]
3. Remove ALL duplicate, outdated, or backup files (.bak, .backup, .old, etc.)
4. Create a single source of truth for each type of information
5. DO NOT stop until you have verified every location

PROCESS YOU MUST FOLLOW:
1. First, run: find . -name "*.md" | grep -v node_modules | grep -v target | sort
2. Read through ALL markdown files to identify which need updates
3. Create a todo list of ALL files that need changes
4. Update each file systematically
5. Remove all redundant/duplicate files
6. Commit and push ALL changes in logical groups
7. Verify the changes are on GitHub

SPECIFIC UPDATE NEEDED:
[Describe what needs to be updated, e.g., "Add GPU benchmark results (72.7M ops/sec) to all relevant documentation"]

FILES TO CHECK (minimum):
- All README.md files (root and subdirectories)
- All BENCHMARK*.md files
- All technical documentation in docs/
- All guides in docs/guides/
- All benchmark docs in docs/benchmarks/
- Implementation summaries
- Any files mentioning performance/benchmarks/results

CONSOLIDATION RULES:
- If multiple files contain the same information, consolidate to ONE authoritative file
- Delete duplicate files after consolidation
- Update all cross-references to point to the consolidated file
- Document what was consolidated in commit messages

VERIFICATION STEPS:
1. grep -r "[KEY TERM]" . to verify updates are present
2. Check for remaining backup files: find . -name "*.bak*" -o -name "*.backup" -o -name "*.old"
3. Review git status to ensure all changes are committed
4. Confirm git push succeeds
5. Check that commit SHA matches between local and remote

DO NOT:
- Skip files without telling me
- Say "I updated the main files" without checking ALL files
- Leave backup files in the repository
- Create new backup files
- Stop until every file is verified updated

REPORT BACK:
After completion, provide:
1. List of ALL files updated (with line counts changed)
2. List of ALL files deleted
3. Final git commit SHAs
4. grep verification showing the update is in all expected files
```

---

## Common Use Cases

### 1. Adding New Benchmark Data

```
I need to add NEW_BENCHMARK_RESULTS to the entire repository.

Data to add:
- [Specific metrics, numbers, dates]
- [Hardware/platform information]
- [Test conditions]

Follow the DOCUMENTATION_CLEANUP_PROMPT.md process.
Check every file in: /, /docs, /docs/benchmarks, /docs/guides, /docs/technical, /benches
Update ALL files that mention benchmarks or performance.
```

### 2. Removing Duplicate Documentation

```
I need to consolidate duplicate documentation files.

Follow the DOCUMENTATION_CLEANUP_PROMPT.md process.
Specifically:
1. Find all duplicate files with similar names or content
2. Merge into ONE authoritative version
3. Delete all duplicates
4. Update all cross-references
5. Document consolidation in README files
```

### 3. Updating Technical Documentation

```
I need to update ALL technical documentation with NEW_INFORMATION.

Information to add:
- [Technical details]
- [Architecture changes]
- [New capabilities]

Follow the DOCUMENTATION_CLEANUP_PROMPT.md process.
Check: docs/technical/*.md, docs/guides/*.md, README.md, IMPLEMENTATION_SUMMARY.md
```

### 4. Cleaning Backup Files

```
I need to remove ALL backup and temporary files from the repository.

Follow the DOCUMENTATION_CLEANUP_PROMPT.md process.
Find and delete: *.bak, *.backup, *.old, *.tmp, *~, *.swp
Search in: entire repository (excluding node_modules, target)
Commit deletion with clear message documenting what was removed.
```

---

## Why This Prompt is Necessary

**Problem:** AI assistants often:
- Update only "obvious" files (like README.md)
- Skip files they don't immediately see
- Don't check for duplicates
- Leave backup files littering the repository
- Don't verify their changes propagated to GitHub
- Say they're "done" when only 20% of files are updated

**Solution:** This prompt:
- Forces systematic checking of ALL files
- Requires verification at each step
- Demands consolidation of duplicates
- Includes cleanup of backup files
- Requires proof of completion (git SHAs, grep results)

---

## Verification Commands

After any documentation update, run these to verify:

```bash
# Check for backup files
find . -name "*.bak*" -o -name "*.backup" -o -name "*.old" 2>/dev/null

# Verify specific content is present
grep -r "SEARCH_TERM" . --include="*.md" | wc -l

# List all markdown files
find . -name "*.md" | grep -v node_modules | grep -v target | sort

# Check git status
git status
git log --oneline -5
git ls-remote origin main
git rev-parse HEAD
```

---

## Example Session

```
USER: "I need to add the new GPU benchmark (72.7M ops/sec) to all documentation"

ASSISTANT: Following DOCUMENTATION_CLEANUP_PROMPT.md...

[Runs find command to get all .md files]
[Creates todo list of 15 files to update]
[Updates each file systematically]
[Removes 6 duplicate files]
[Commits in logical groups]
[Verifies with grep that "72.7M" appears in all expected files]
[Confirms git push with matching SHAs]

RESULT: 
- 15 files updated
- 6 duplicates removed
- 4 commits pushed
- Verification: grep shows "72.7M" in 18 locations
- Git SHA matches: local(abc123) == remote(abc123) ✓
```

---

## File Organization Best Practices

After cleanup, maintain this structure:

```
Repository Root:
├── README.md                    # Main entry point
├── BENCHMARK_DATA.md            # Executive summary (CPU + GPU)
├── IMPLEMENTATION_SUMMARY.md    # Implementation details
│
docs/
├── README.md                    # Docs navigation hub
├── agents.md                    # AI agent integration
│
├── benchmarks/
│   ├── README.md               # Benchmark navigation (CENTRAL HUB)
│   ├── GPU_L4_RESULTS.md       # All GPU data (SINGLE SOURCE)
│   ├── COMPARATIVE_ANALYSIS.md  # Cross-tool comparisons
│   └── data_exports/           # Raw data only
│
├── guides/
│   ├── overview.md             # Product overview
│   ├── how-it-works.md         # Simple explanation
│   └── docker.md               # Deployment guide
│
└── technical/
    ├── architecture.md          # System design
    ├── algorithms.md            # Algorithm details
    └── scientific-overview.md   # Academic reference
```

**Rules:**
1. ONE authoritative file per topic
2. NO duplicate files
3. NO backup files committed
4. Clear navigation hubs (README.md files)
5. Cross-references point to authoritative sources

---

## Anti-Patterns to Avoid

❌ **Multiple files with same content**
```
BENCHMARK_DATA.md
docs/benchmarks/BENCHMARK_DATA.md  # DUPLICATE!
docs/BENCHMARK_DATA.md             # DUPLICATE!
```

❌ **Backup files in repository**
```
README.md.bak
README.md.old
README.md.backup
Cargo.toml.broken
```

❌ **Unclear file purposes**
```
results.md
new_results.md
final_results.md
results_v2.md
```

❌ **Meta-documentation about finding docs**
```
HOW_TO_FIND_BENCHMARKS.md
WHERE_IS_THE_DATA.md
SYNCING_INSTRUCTIONS.md
```

✅ **Instead: Clear navigation hubs**
```
docs/benchmarks/README.md  # Links to all benchmark docs with descriptions
```

---

## Success Criteria

After using this prompt, you should have:

✅ Single source of truth for each type of information
✅ No duplicate files with similar content
✅ No backup files (*.bak, *.backup, *.old)
✅ Clear navigation hubs (README.md files)
✅ All cross-references working
✅ All changes committed and pushed
✅ Verification that content appears in all expected locations
✅ Matching git SHAs between local and remote

---

**Last Updated:** 2025-11-08
**Use this prompt ANY TIME you need comprehensive documentation updates.**
