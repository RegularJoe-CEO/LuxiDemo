# Documentation Update Protocol

**Append this to every implementation request to ensure all documentation stays synchronized.**

---

## Required Documentation Updates

After implementing any feature, bug fix, or performance improvement, you MUST update all affected documentation files to maintain consistency across the repository. This is a **mandatory step** before completing any task.

### 1. XAI Documents (Always Update)

Update these files whenever ANY change is made to the codebase:

- **`docs/XAI_EXECUTIVE_SUMMARY.md`**
  - Add new features to the "Latest" section at the end
  - Update performance metrics if changed
  - Add references to new documentation files
  - Update platform support matrix if applicable
  
- **`docs/benchmarks/xai_integration.md`**
  - Add new benchmark results
  - Update performance tables
  - Add examples of new features
  - Include energy efficiency data if applicable

- **`docs/benchmarks/xai_escalation_plan.md`**
  - Add new capabilities to "Latest Updates" section
  - Update status of existing features
  - Add relevance to xAI use cases
  - Update documentation references

### 2. Main Route Documentation (docs/)

Update the main documentation index:

- **`docs/README.md`**
  - Add announcement section for significant new features (after GPU section)
  - Update all performance metrics in tables
  - Add cross-references to new documentation files
  - Update the "Documentation Structure" section if new docs are added
  - Ensure all file paths are correct

### 3. Benchmark Route Documentation (docs/benchmarks/)

Update the benchmark navigation hub:

- **`docs/benchmarks/README.md`**
  - Add new benchmark results to "Latest Results" section
  - Update performance comparison tables
  - Add references to new benchmark documentation
  - Update "Running Benchmarks" section with new commands
  - Add new documents to "Quick Links" table

### 4. Root-Level Documentation

Update these files in the repository root:

- **`BENCHMARK_DATA.md`**
  - Add new benchmark sections with results
  - Update performance tables
  - Add "Documentation" subsections with links to new docs
  - Update energy efficiency data
  - Add usage examples for new features

- **`IMPLEMENTATION_SUMMARY.md`**
  - Add new implementation sections
  - Update "Achievement" checklists
  - Add "New Documentation" section with links
  - Update performance summary tables
  - Update "Last Updated" timestamp

- **`README.md`** (if performance metrics change)
  - Update key performance numbers
  - Update feature lists
  - Add references to major new capabilities

### 5. Cross-Reference Validation

After updating documentation, verify:

- [ ] All internal links work (relative paths correct)
- [ ] New documentation files are referenced from at least 2-3 other docs
- [ ] Performance metrics are consistent across all files
- [ ] Dates are updated ("November XX, 2025" format)
- [ ] No duplicate information (follow single source of truth principle)

### 6. Single Source of Truth Principle

**Remember:**
- Each fact should have ONE authoritative location
- Other documents should LINK to that location, not duplicate content
- Update the source, then update all references

**Example Structure:**
```
BENCHMARK_DATA.md (source of truth for performance data)
  ├─> docs/README.md (references BENCHMARK_DATA.md)
  ├─> docs/benchmarks/README.md (references BENCHMARK_DATA.md)
  └─> docs/XAI_EXECUTIVE_SUMMARY.md (references BENCHMARK_DATA.md)
```

### 7. Documentation Checklist Template

Use this checklist for EVERY code change:

```markdown
## Documentation Updates

### XAI Documents
- [ ] Updated docs/XAI_EXECUTIVE_SUMMARY.md
- [ ] Updated docs/benchmarks/xai_integration.md
- [ ] Updated docs/benchmarks/xai_escalation_plan.md

### Main Route (docs/)
- [ ] Updated docs/README.md
- [ ] Added cross-references to new documentation

### Benchmark Route (docs/benchmarks/)
- [ ] Updated docs/benchmarks/README.md
- [ ] Updated performance tables

### Root-Level
- [ ] Updated BENCHMARK_DATA.md
- [ ] Updated IMPLEMENTATION_SUMMARY.md
- [ ] Updated README.md (if needed)

### Validation
- [ ] Verified all links work
- [ ] Ensured metrics are consistent
- [ ] Updated all timestamps
- [ ] No duplicate content
```

---

## Quick Reference: What to Update When

### When Adding New Features
1. ✅ XAI_EXECUTIVE_SUMMARY.md - Add to "Latest" section
2. ✅ docs/README.md - Add announcement section
3. ✅ IMPLEMENTATION_SUMMARY.md - Add implementation details
4. ✅ All XAI docs - Add use cases and examples

### When Adding New Benchmarks
1. ✅ BENCHMARK_DATA.md - Add results section
2. ✅ docs/benchmarks/README.md - Add to "Latest Results"
3. ✅ xai_integration.md - Add benchmark data
4. ✅ xai_escalation_plan.md - Add performance metrics

### When Adding New Documentation Files
1. ✅ docs/README.md - Add to structure section
2. ✅ BENCHMARK_DATA.md - Add references
3. ✅ IMPLEMENTATION_SUMMARY.md - Add to "New Documentation"
4. ✅ XAI_EXECUTIVE_SUMMARY.md - Add reference links
5. ✅ docs/benchmarks/README.md - Add to navigation

### When Changing Performance Metrics
1. ✅ Update ALL files with that metric
2. ✅ Check for consistency across repository
3. ✅ Update comparison tables
4. ✅ Verify economic impact calculations

---

## Automation Script (Future Enhancement)

Consider creating a script to automate consistency checks:

```bash
#!/bin/bash
# Future: tools/verify_doc_consistency.sh

echo "Checking documentation consistency..."

# Check for broken internal links
# Check for outdated timestamps
# Check for metric inconsistencies
# Check for missing cross-references
```

---

**Last Updated:** 2025-11-10  
**Owner:** Documentation Team  
**Review:** Required for all PRs affecting features/performance
