# AI Agent Prompts — Quick Reference

**Repository:** https://github.com/RegularJoe-CEO/LuxiEdge  
**Last Updated:** 2025-11-08

---

## Which Prompt Do I Need?

### 🤖 I want to validate performance claims from the website
**Use:** `AI_AGENT_VALIDATION_PROMPT.md`

**Purpose:** Complete validation of Luxi Edge performance claims for users who discovered the tool online and need independent verification before deployment.

**User Journey:** Website discovery → Repository validation → Benchmark execution → Decision support

**Outputs:**
- Website claims verification (GPU 72.7M ops/sec, CPU 1.6ms)
- Confidence assessment (HIGH/MEDIUM/LOW)
- Clear deployment recommendation
- Economic ROI validation

---

### 🏗️ I need to update the live Replit website
**Use:** `REPLIT_WEBSITE_UPDATE_PROMPT.md`

**Purpose:** Deploy updated HTML/CSS to the live marketing website at https://e-rock-compute-eric589.replit.app/

**User Journey:** GitHub repo update → Manual Replit sync → Website verification

**Outputs:**
- Updated live website
- New marketing content
- AI agent navigation guide on website

---

### 📖 I want to understand the agent validation protocol
**Use:** `AGENTS.md`

**Purpose:** Technical specification for automated benchmark validation and artifact generation.

**User Journey:** Developer integration → Automated validation → Standardized artifacts

**Outputs:**
- bench_results.json (machine-readable)
- bench_results.csv (tabular)
- bench_summary.md (human-readable)
- economics_summary.md (CFO one-pager)

---

### 🔧 I encountered validation failures
**Use:** `AGENT_TROUBLESHOOTING.md`

**Purpose:** Recovery guide for common failure modes when validating the repository.

**User Journey:** Validation attempt → Failure → Troubleshooting → Success

**Outputs:**
- Root cause identification
- Step-by-step recovery
- FAQ for common issues
- Success criteria clarification

---

### 📊 I want to see successful validation results
**Use:** `AGENT_VALIDATION_COMPLETE.md`

**Purpose:** Example of successful validation execution with HIGH confidence results.

**Outputs:**
- Proof that validation works
- Example results and confidence assessment
- Deployment readiness recommendation

---

### 🔍 I need to respond to failed validation reports
**Use:** `RESPONSE_TO_GROK_FAILURES.md`

**Purpose:** Evidence that repository is complete and functional, addressing specific failure claims.

**Outputs:**
- Point-by-point rebuttal of failure claims
- Verification that all files exist
- Root cause analysis of failures

---

## Quick Decision Tree

```
START
  │
  ├─ "I found Luxi Edge online and want to verify claims"
  │   → Use AI_AGENT_VALIDATION_PROMPT.md
  │
  ├─ "I need to update the marketing website"
  │   → Use REPLIT_WEBSITE_UPDATE_PROMPT.md
  │
  ├─ "I'm integrating agent validation into my workflow"
  │   → Use AGENTS.md
  │
  ├─ "My validation failed and I need help"
  │   → Use AGENT_TROUBLESHOOTING.md
  │
  ├─ "I want to see proof that validation works"
  │   → Use AGENT_VALIDATION_COMPLETE.md
  │
  └─ "Someone reported failures and I need to respond"
      → Use RESPONSE_TO_GROK_FAILURES.md
```

---

## Document Purposes Summary

| Document | Primary Audience | Purpose | Output |
|----------|-----------------|---------|--------|
| **AI_AGENT_VALIDATION_PROMPT.md** | End users & AI agents | Complete user journey validation | Decision-ready report |
| **REPLIT_WEBSITE_UPDATE_PROMPT.md** | Replit deployment agents | Website HTML/CSS updates | Live website deployment |
| **AGENTS.md** | Developer automation | Technical validation spec | Standardized artifacts |
| **AGENT_TROUBLESHOOTING.md** | Failing agents | Error recovery guide | Successful validation |
| **AGENT_VALIDATION_COMPLETE.md** | Stakeholders | Proof of validation success | Confidence in claims |
| **RESPONSE_TO_GROK_FAILURES.md** | Technical reviewers | Addressing failure reports | Repository credibility |
| **SUMMARY_OF_ACTIONS.md** | Project managers | Timeline of improvements | Audit trail |

---

## Typical Workflows

### For Users Who Found the Website:

1. Visit https://e-rock-compute-eric589.replit.app/
2. See AI agent validation instructions
3. Agent uses: `AI_AGENT_VALIDATION_PROMPT.md`
4. User receives decision-ready report
5. User makes informed deployment decision

### For Developers Integrating Validation:

1. Read: `AGENTS.md` for technical spec
2. Run: `./scripts/quickbench.sh`
3. Consume: `artifacts/bench_results.json` programmatically
4. If issues: Consult `AGENT_TROUBLESHOOTING.md`

### For Marketing/Website Updates:

1. Update `holding/marketing-site/index.html` in GitHub
2. Agent uses: `REPLIT_WEBSITE_UPDATE_PROMPT.md`
3. Manually sync to Replit
4. Verify live site matches

---

## All Prompts Available At:

**Repository:** https://github.com/RegularJoe-CEO/LuxiEdge

**Direct Links:**
- AI_AGENT_VALIDATION_PROMPT.md: https://github.com/RegularJoe-CEO/LuxiEdge/blob/main/AI_AGENT_VALIDATION_PROMPT.md
- REPLIT_WEBSITE_UPDATE_PROMPT.md: https://github.com/RegularJoe-CEO/LuxiEdge/blob/main/REPLIT_WEBSITE_UPDATE_PROMPT.md
- AGENTS.md: https://github.com/RegularJoe-CEO/LuxiEdge/blob/main/AGENTS.md
- AGENT_TROUBLESHOOTING.md: https://github.com/RegularJoe-CEO/LuxiEdge/blob/main/AGENT_TROUBLESHOOTING.md

---

**Version:** 1.0  
**Maintainer:** Eric Waller (e@ewaller.com)  
**License:** LicenseRef-Luxi-Business-1.0
