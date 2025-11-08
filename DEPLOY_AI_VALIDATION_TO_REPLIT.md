# Replit Agent: Deploy AI Agent Validation Links

**Priority:** HIGH  
**Date Needed:** Immediately  
**Estimated Time:** 5 minutes

---

## What Needs to Happen

The GitHub repository has been updated with prominent AI agent validation links, but the **live Replit website** has NOT been updated yet.

**Live Website:** https://e-rock-compute-eric589.replit.app/  
**Updated HTML Source:** https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/holding/marketing-site/index.html  
**Last GitHub Commit:** b532eb1 (2025-11-08)

---

## What Changed

**NEW CONTENT ADDED:**
1. Prominent highlighted section: "🤖 For AI Agents: Performance Validation"
2. Direct link to UNIVERSAL_AGENT_VALIDATION_PROMPT.md (copy-paste ready for AI agents)
3. Updated Resources section to list AI validation prompt as FIRST item
4. Updated Running Benchmarks section with universal validation link

**GOAL:**
Make it trivially easy for AI agents visiting the website to find the validation prompt and copy-paste it to validate performance claims.

---

## Step-by-Step Deployment

### Step 1: Get the Updated HTML

Copy the entire raw HTML from this URL:
```
https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/holding/marketing-site/index.html
```

**To verify you have the right version:**
The HTML should contain this text (search for it):
- `"🤖 For AI Agents: Performance Validation"`
- `"UNIVERSAL_AGENT_VALIDATION_PROMPT.md"`
- `"Copy-Paste Validation Prompt"`

If you see these, you have the correct updated version. ✅

If you DON'T see these, you have the old version. ❌

---

### Step 2: Open Replit Project

1. Go to your Replit workspace for the Luxi Edge website
2. Find the file that serves the homepage (likely `index.html` at the root or in a `public/` folder)
3. Open that file in the editor

---

### Step 3: Replace ALL Content

1. Select ALL content in your current index.html (Ctrl+A / Cmd+A)
2. Delete it
3. Paste the entire new HTML you copied from GitHub
4. Save the file

**CRITICAL:** Replace the ENTIRE file content, not just parts of it.

---

### Step 4: Verify the Update Locally

Before deploying, check the file contains:

**Search for these exact strings in your updated file:**

```
🤖 For AI Agents: Performance Validation
```

```
UNIVERSAL_AGENT_VALIDATION_PROMPT.md
```

```
Works with: ChatGPT (Advanced Data Analysis), Claude (computer use)
```

**If you find all three:** ✅ Update is correct, proceed to Step 5  
**If any are missing:** ❌ You pasted the wrong file, try Step 1 again

---

### Step 5: Deploy to Live Site

1. Save your changes in Replit
2. If Replit requires a manual deploy/restart, do that now
3. Wait for the site to rebuild/redeploy

---

### Step 6: Verify Live Website

**Open in browser:** https://e-rock-compute-eric589.replit.app/

**Check #1: View Page Source**
- Right-click → "View Page Source" (or Ctrl+U / Cmd+Option+U)
- Search for: `UNIVERSAL_AGENT_VALIDATION_PROMPT`

**Expected:** ✅ Found (multiple times)  
**If not found:** ❌ Deployment didn't work, check Replit deployment status

---

**Check #2: Visual Inspection**

Scroll down the page and look for:

✅ **NEW highlighted blue box with:**
```
🤖 For AI Agents: Performance Validation
📋 Copy-Paste Validation Prompt
```

✅ **Resources section should START with:**
```
Resources
• 🤖 AI Agent Validation Prompt — Copy-paste to any AI to validate performance claims
```

✅ **Running Benchmarks section should include:**
```
• Universal Validation: UNIVERSAL_AGENT_VALIDATION_PROMPT.md (copy-paste to any AI agent)
```

---

**Check #3: Test Links**

Click this link on the live page:
```
🤖 AI Agent Validation Prompt
```

**Expected:** Should open GitHub raw file showing validation prompt text  
**URL should be:** https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/UNIVERSAL_AGENT_VALIDATION_PROMPT.md

---

### Step 7: Confirm Deployment

**If all checks pass:**

Report back:
> ✅ Deployment complete. Live website updated at https://e-rock-compute-eric589.replit.app/. AI agent validation links are now visible and functional.

**If any checks fail:**

Report what you see:
> ⚠️ Deployment issue: [describe what's not working]. Page source [does/doesn't] contain validation links. Visual inspection [shows/doesn't show] highlighted section.

---

## What You Should See (Before vs After)

### BEFORE (Current Live Site)
```
🤖 For AI Agents: GitHub Navigation Guide

[Long list of documentation links]
[Links to AGENTS.md - technical spec]
```

### AFTER (Updated Site)
```
🤖 For AI Agents: Performance Validation

[Highlighted blue box]
📋 Copy-Paste Validation Prompt
Universal Agent Validation: UNIVERSAL_AGENT_VALIDATION_PROMPT.md
Works with: ChatGPT, Claude, Copilot, Cursor, Replit, etc.

[What gets validated]
✅ Code builds and runs
✅ CPU SIMD performance
[etc.]

📊 Benchmarks & Performance Data
[Rest of navigation guide]
```

---

## Common Issues & Solutions

### Issue 1: "I can't find index.html in Replit"

**Solution:**
- Check for `public/index.html`
- Check for `www/index.html`
- Check Replit configuration file (.replit) to see which file is served
- Look at what file is currently being displayed when you visit the live URL

---

### Issue 2: "Changes saved but don't appear on live site"

**Solution:**
1. Hard refresh browser: Ctrl+Shift+R (Windows) or Cmd+Shift+R (Mac)
2. Open incognito/private window
3. Check if Replit needs manual deployment trigger
4. Restart the Repl entirely

---

### Issue 3: "Site looks broken after update"

**Cause:** Missing `styles.css` file

**Solution:**
Ensure `styles.css` exists in the same directory as `index.html`. If missing, copy it from:
```
https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/holding/marketing-site/styles.css
```

---

### Issue 4: "Old content still shows in view source"

**Cause:** Browser cache or CDN cache

**Solution:**
1. Clear browser cache completely
2. Open in incognito mode
3. Check Replit deployment logs
4. Verify file was actually saved (check last modified timestamp)

---

## Verification Checklist

Before reporting "done", confirm ALL of these:

- [ ] Opened https://e-rock-compute-eric589.replit.app/ in browser
- [ ] Page source contains "UNIVERSAL_AGENT_VALIDATION_PROMPT" (Ctrl+F to search)
- [ ] Visually see highlighted blue box with "🤖 For AI Agents: Performance Validation"
- [ ] Resources section lists "🤖 AI Agent Validation Prompt" as FIRST item
- [ ] Clicked validation prompt link - opens GitHub raw file with prompt text
- [ ] Hard refreshed browser (Ctrl+Shift+R) to clear cache
- [ ] Tested link: https://raw.githubusercontent.com/.../UNIVERSAL_AGENT_VALIDATION_PROMPT.md loads
- [ ] Mobile/narrow viewport: validation section still visible and readable

---

## Why This Matters

**Current Situation:**
- User visits live Replit website
- Agent sees old "GitHub Navigation Guide" section
- Agent has to click through multiple links to find validation instructions
- Validation prompt (UNIVERSAL_AGENT_VALIDATION_PROMPT.md) is buried

**After This Deployment:**
- User visits live Replit website
- Agent immediately sees "🤖 For AI Agents: Performance Validation" in highlighted box
- Direct link to copy-pasteable validation prompt (raw GitHub URL)
- One click to get the full prompt text
- Works with any AI service (ChatGPT, Claude, Copilot, etc.)

**Impact:**
- Dramatically easier for AI agents to discover validation protocol
- More validation reports from independent agents
- Higher user confidence in performance claims

---

## Direct Links for Copy-Paste

**Updated HTML Source:**
```
https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/holding/marketing-site/index.html
```

**Validation Prompt (what users will click):**
```
https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/UNIVERSAL_AGENT_VALIDATION_PROMPT.md
```

**Live Website to Verify:**
```
https://e-rock-compute-eric589.replit.app/
```

---

## Summary

**Task:** Update live Replit website with new AI agent validation links  
**Method:** Copy updated HTML from GitHub, paste into Replit's index.html, deploy  
**Verification:** Live site shows highlighted validation section and functional links  
**Time:** ~5 minutes  
**Risk:** LOW (static HTML, no backend changes, easy to revert)

---

**Last GitHub Update:** 2025-11-08 (commit b532eb1)  
**Status:** ⚠️ GitHub updated, Replit deployment pending
