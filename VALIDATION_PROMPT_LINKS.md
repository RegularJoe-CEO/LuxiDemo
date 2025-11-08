# Validation Prompt Discovery — Link Checklist

**Last Updated:** 2025-11-08  
**Goal:** Ensure AI agents can easily find the universal validation prompt from any entry point

---

## ✅ All Places Where Validation Prompt is Linked

### 1. **Website (Replit) — PRIMARY ENTRY POINT**
**File:** `holding/marketing-site/index.html`

**Links:**
- ✅ Prominent highlighted section: "🤖 For AI Agents: Performance Validation"
- ✅ Resources section (first item): "🤖 AI Agent Validation Prompt"
- ✅ Running Benchmarks section: "Universal Validation" link

**URL:** https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/UNIVERSAL_AGENT_VALIDATION_PROMPT.md

**Agent Journey:**
1. User visits https://e-rock-compute-eric589.replit.app/
2. Agent sees "🤖 For AI Agents: Performance Validation" section
3. Agent clicks raw GitHub link to UNIVERSAL_AGENT_VALIDATION_PROMPT.md
4. Agent can copy entire prompt directly

---

### 2. **GitHub README — SECONDARY ENTRY POINT**
**File:** `README.md`

**Section:** "AI Agent Validation"

**Link:** Direct link to UNIVERSAL_AGENT_VALIDATION_PROMPT.md

**Agent Journey:**
1. User visits https://github.com/RegularJoe-CEO/LuxiEdge
2. README has "AI Agent Validation" section
3. Link to validation prompt file
4. Lists compatible AI services

---

### 3. **Agent Prompts Index — NAVIGATION HUB**
**File:** `AGENT_PROMPTS_INDEX.md`

**Purpose:** Help users choose correct prompt

**Highlights:**
- ⭐ UNIVERSAL_AGENT_VALIDATION_PROMPT.md as recommended starting point
- Decision tree for choosing prompts
- Links to all validation documentation

---

### 4. **Repository Root — DIRECT ACCESS**
**File:** `UNIVERSAL_AGENT_VALIDATION_PROMPT.md`

**URL (formatted):** https://github.com/RegularJoe-CEO/LuxiEdge/blob/main/UNIVERSAL_AGENT_VALIDATION_PROMPT.md

**URL (raw - copy-paste ready):** https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/UNIVERSAL_AGENT_VALIDATION_PROMPT.md

**Agent Journey:**
1. User browses repository files
2. Sees UNIVERSAL_AGENT_VALIDATION_PROMPT.md at root level
3. Clicks to open
4. Copies entire file

---

## 🎯 Discovery Paths for AI Agents

### Path 1: Website → Raw Prompt (FASTEST)
```
https://e-rock-compute-eric589.replit.app/
  → "🤖 For AI Agents: Performance Validation" section
  → Click raw URL: https://raw.githubusercontent.com/.../UNIVERSAL_AGENT_VALIDATION_PROMPT.md
  → Copy entire prompt
  → Paste to AI service
```

### Path 2: Website → GitHub → Prompt
```
https://e-rock-compute-eric589.replit.app/
  → "GitHub Repository" link
  → README.md "AI Agent Validation" section
  → UNIVERSAL_AGENT_VALIDATION_PROMPT.md link
  → Copy prompt
```

### Path 3: GitHub Direct → Prompt
```
https://github.com/RegularJoe-CEO/LuxiEdge
  → README.md "AI Agent Validation" section
  → UNIVERSAL_AGENT_VALIDATION_PROMPT.md link
  → Copy prompt
```

### Path 4: Search → Direct File
```
Google/Search: "LuxiEdge AI agent validation"
  → GitHub result
  → UNIVERSAL_AGENT_VALIDATION_PROMPT.md file
  → Copy prompt
```

---

## 📍 Visual Location Summary

### Website HTML Structure
```html
<main>
  <!-- ... Quick Start, Resources ... -->
  
  <section class="card highlight">
    <h2>🤖 For AI Agents: Performance Validation</h2>
    <div style="...highlighted box...">
      <h3>📋 Copy-Paste Validation Prompt</h3>
      <p><a href="...UNIVERSAL_AGENT_VALIDATION_PROMPT.md">...</a></p>
      ^^^^^^ PRIMARY LINK ^^^^^^^
    </div>
  </section>
  
  <!-- ... Navigation Guide ... -->
</main>
```

### Resources Section
```html
<section class="card">
  <h2>Resources</h2>
  <ul class="resources">
    <li><a href="...UNIVERSAL_AGENT_VALIDATION_PROMPT.md">🤖 AI Agent Validation Prompt</a></li>
    ^^^^^^ FIRST RESOURCE ^^^^^^^
    <li><a href="...">GitHub Repository</a></li>
    <li>...</li>
  </ul>
</section>
```

---

## ✅ Verification Checklist

**To verify all links work:**

- [ ] Open https://e-rock-compute-eric589.replit.app/
- [ ] Find "🤖 For AI Agents: Performance Validation" section (should be highlighted)
- [ ] Click raw URL link: https://raw.githubusercontent.com/.../UNIVERSAL_AGENT_VALIDATION_PROMPT.md
- [ ] Verify prompt loads in browser (should show full markdown text)
- [ ] Check Resources section has "🤖 AI Agent Validation Prompt" as FIRST item
- [ ] Open https://github.com/RegularJoe-CEO/LuxiEdge
- [ ] Find "AI Agent Validation" section in README
- [ ] Click UNIVERSAL_AGENT_VALIDATION_PROMPT.md link
- [ ] Verify file opens on GitHub

---

## 🚨 What to Check if Agent Can't Find Prompt

### If agent says "I can't find the validation instructions":

**1. Check Replit site is updated:**
```bash
curl https://e-rock-compute-eric589.replit.app/ | grep "UNIVERSAL_AGENT_VALIDATION_PROMPT"
# Should return matches
```

**2. Check GitHub has latest:**
```bash
curl https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/UNIVERSAL_AGENT_VALIDATION_PROMPT.md
# Should return full prompt (HTTP 200)
```

**3. Check README has section:**
```bash
curl https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/README.md | grep "AI Agent Validation"
# Should return section header
```

**4. Provide direct copy-paste URL:**
```
https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/UNIVERSAL_AGENT_VALIDATION_PROMPT.md
```

---

## 📱 Mobile/Accessibility Notes

**Website is responsive:**
- Highlighted validation section visible on mobile
- Links are tappable/clickable
- Raw GitHub URL works on all devices

**For screen readers:**
- Section has clear heading: "For AI Agents: Performance Validation"
- Links have descriptive text
- List structure for navigation items

---

## 🔄 Update Protocol

**When updating the validation prompt:**

1. Edit `UNIVERSAL_AGENT_VALIDATION_PROMPT.md`
2. Commit and push to GitHub
3. No website HTML changes needed (links to raw URL - auto-updates)
4. Verify with: `curl https://raw.githubusercontent.com/.../UNIVERSAL_AGENT_VALIDATION_PROMPT.md`

**When updating website:**

1. Edit `holding/marketing-site/index.html`
2. Commit and push to GitHub
3. **Manually update Replit** (use REPLIT_WEBSITE_UPDATE_PROMPT.md)
4. Verify live site shows new content

---

## 📊 Link Analytics (if needed)

**Track which path agents use most:**
- Website → Raw URL (direct copy)
- Website → GitHub → File
- GitHub README → File
- Direct repository browsing

*Note: GitHub doesn't provide analytics for raw URLs, but commit history shows when prompt is accessed via git clone.*

---

## Summary: Agent Discovery is Optimized ✅

**An AI agent visiting the website will see:**
1. ✅ Prominent highlighted section at the top
2. ✅ Direct raw URL to copy-paste prompt
3. ✅ List of compatible AI services
4. ✅ Clear "what gets validated" checklist
5. ✅ First item in Resources section
6. ✅ Multiple paths to same prompt

**An AI agent visiting GitHub will see:**
1. ✅ "AI Agent Validation" section in README
2. ✅ Direct link to universal prompt file
3. ✅ File visible in repository root
4. ✅ Navigation index (AGENT_PROMPTS_INDEX.md)

**Result:** No matter where an agent enters (website or GitHub), they can find the validation prompt in ≤2 clicks.

---

**Last Verified:** 2025-11-08 (commit b532eb1)  
**Status:** ✅ ALL LINKS ACTIVE AND WORKING
