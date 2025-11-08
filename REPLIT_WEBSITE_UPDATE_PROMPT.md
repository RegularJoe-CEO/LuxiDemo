# Replit Agent: Website Update Instructions

## ⚠️ IMPORTANT: Replit Site is NOT Auto-Synced with GitHub

The live website at https://e-rock-compute-eric589.replit.app/ is **NOT automatically connected** to the GitHub repository. You need to **manually copy** the updated HTML content to the Replit project.

---

## Objective
Update the Luxi Edge marketing website to match the newly redesigned GitHub landing page.

**GitHub Repository:** https://github.com/RegularJoe-CEO/LuxiEdge  
**Live Website:** https://e-rock-compute-eric589.replit.app/  
**Updated HTML Source:** https://github.com/RegularJoe-CEO/LuxiEdge/blob/main/holding/marketing-site/index.html

---

## Option 1: Manual Copy (Recommended if Replit is not git-connected)

### Step 1: Get the Updated HTML
Visit: https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/holding/marketing-site/index.html

Copy the entire raw HTML content.

### Step 2: Update Replit's index.html
In your Replit project:
1. Open the `index.html` file (or whatever file serves the homepage)
2. **Replace all content** with the copied HTML
3. Save the file

### Step 3: Verify the Update
The new HTML should start with:
```html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>Luxi Edge — Ultra-Efficient Mathematical Computation</title>
```

NOT:
```html
<title>Luxi Edge — Engineer Preview</title>
```

---

## Option 2: Git Clone (If Replit supports git)

### Step 1: Check if Replit has Git Access
### Step 1: Check if Replit has Git Access
In the Replit shell, run:
```bash
git status
```

**If you get a repository status:** Replit is connected to git. Proceed to Step 2.  
**If you get "not a git repository":** Use Option 1 (Manual Copy) instead.

### Step 2: Pull Latest Changes

### Step 2: Pull Latest Changes
```bash
git pull origin main
```

### Step 3: Copy Files to Web Root
The updated HTML is in `holding/marketing-site/index.html`. Copy it to your web root:
```bash
# If Replit serves from root directory:
cp holding/marketing-site/index.html ./index.html

# Or if Replit has a public/www directory:
cp holding/marketing-site/index.html ./public/index.html
```

### Step 4: Copy Styles (if needed)
```bash
cp holding/marketing-site/styles.css ./styles.css
# Or: cp holding/marketing-site/styles.css ./public/styles.css
```

---

## What Changed in the HTML

### OLD Content (Currently Live)
### OLD Content (Currently Live)
```html
<title>Luxi Edge — Engineer Preview</title>
<div class="tag">World's fastest edge numerical microservice</div>
```
- Metrics: "13.7× faster", "193k ops/sec", "596mW under load"
- Positioning: "Engineer Preview"
- Links to: readme.html, iot.html, ml.html, data.html, edge.html, musk.html

### NEW Content (Should Be Updated To)
```html
<title>Luxi Edge — Ultra-Efficient Mathematical Computation</title>
<div class="tag">Ultra-Efficient Mathematical Computation at Scale</div>
<p class="subtitle">Energy-first compute platform delivering >10× performance and >5× energy efficiency</p>
```
- Metrics: "72.7M ops/sec" (GPU), "1.6ms for 100K elements" (CPU)
- Positioning: "Production-ready computational acceleration platform"
- Links to: GitHub repository, docs, benchmarks, xAI integration guide

---

## How to Verify the Update Worked

### Method 1: View Page Source
Visit https://e-rock-compute-eric589.replit.app/ and view source (Ctrl+U or Cmd+Option+U).

**Search for:** `72.7M ops/sec`  
- ✅ **Found:** Update successful!  
- ❌ **Not found:** Update failed, HTML wasn't copied correctly

**Search for:** `193k ops/sec`  
- ❌ **Found:** Still showing old content  
- ✅ **Not found:** Old content removed successfully

### Method 2: Check Page Title
Browser tab should show: **"Luxi Edge — Ultra-Efficient Mathematical Computation"**  
NOT: "Luxi Edge — Engineer Preview"

### Method 3: Visual Check
The page should have:
- ✅ Four use case cards (Industrial, AI/ML, Autonomous, Energy-Critical)
- ✅ GPU performance table showing 72.7M ops/sec
- ✅ Platform support matrix (x86_64, ARM64, NVIDIA GPU)
- ✅ Enterprise & Strategic Partnerships section
- ✅ Links to GitHub (not to readme.html, iot.html, etc.)

---

## Common Issues & Solutions

### Issue 1: "I copied the HTML but the site looks broken"
**Cause:** Missing `styles.css` file  
**Solution:** Copy the CSS from the GitHub repo or use the template below

### Issue 2: "Links to readme.html, iot.html still appear"
**Cause:** Wrong HTML file was updated  
**Solution:** 
1. Find which file Replit is actually serving (check Replit config)
2. Update THAT file, not just any index.html
3. Check if Replit uses `public/index.html` or root `index.html`

### Issue 3: "Changes don't appear after refresh"
**Cause:** Browser cache or CDN cache  
**Solution:**
1. Hard refresh: Ctrl+Shift+R (Windows) or Cmd+Shift+R (Mac)
2. Open incognito/private window
3. Check Replit deployment status (may need to restart Repl)

### Issue 4: "Git pull says 'not a git repository'"
**Cause:** Replit is not connected to GitHub  
**Solution:** Use Option 1 (Manual Copy) instead

---

## Quick Checklist for Replit Agent

Before you report "done", verify ALL of these:

- [ ] Opened https://e-rock-compute-eric589.replit.app/ in browser
- [ ] Page title is "Luxi Edge — Ultra-Efficient Mathematical Computation" (NOT "Engineer Preview")
- [ ] Searched page source for "72.7M ops/sec" — FOUND
- [ ] Searched page source for "193k ops/sec" — NOT FOUND
- [ ] Searched page source for "596mW" — NOT FOUND  
- [ ] Searched page source for "13.7×" — NOT FOUND
- [ ] GPU performance table is visible on page
- [ ] Platform support matrix is visible on page
- [ ] "Enterprise & Strategic Partnerships" section exists
- [ ] Links point to GitHub (https://github.com/RegularJoe-CEO/LuxiEdge)
- [ ] NO links to "readme.html", "iot.html", "ml.html" on the homepage
- [ ] Tested on mobile/narrow viewport (responsive design works)

---

## Why This Happened

The Replit website is **separate** from the GitHub repository. When we updated `holding/marketing-site/index.html` in GitHub, it didn't automatically update the live Replit site.

**Two separate things:**
1. **GitHub repo** (`/holding/marketing-site/index.html`) — Updated ✅
2. **Replit website** (live at replit.app URL) — Needs manual update ❌

This prompt helps you sync them.

---

## Direct Link to Updated HTML

**Raw file URL:**  
https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/holding/marketing-site/index.html

**Formatted view:**  
https://github.com/RegularJoe-CEO/LuxiEdge/blob/main/holding/marketing-site/index.html

Copy the raw version and paste it into Replit's HTML file.

---

### REMOVED (Old Metrics)
- ❌ "13.7× faster" (outdated CPU-only comparison)
- ❌ "193k ops/sec" (superseded by GPU benchmarks)
- ❌ "596mW under load" (specific to old configuration)
- ❌ "Engineer Preview" positioning

### ADDED (New Content)
- ✅ "72.7M ops/sec" GPU performance (NVIDIA L4)
- ✅ "1.6ms for 100K elements" CPU SIMD performance
- ✅ ">10× performance, >5× energy efficiency" value proposition
- ✅ Four use case sections with icons
- ✅ Platform support matrix (Production/Validated/Planned status)
- ✅ Enterprise & Strategic Partnerships section
- ✅ NDA Partner Program callout
- ✅ Quick Start with HTTP API example
- ✅ Resources section with GitHub links

---

## Troubleshooting

### If the site doesn't update:
1. **Check git status:** `git status` (ensure you're on main branch)
2. **Force pull:** `git reset --hard origin/main`
3. **Clear browser cache:** Hard refresh (Ctrl+Shift+R or Cmd+Shift+R)
4. **Check file path:** Verify server is serving from `holding/marketing-site/`
5. **Restart Replit:** Stop and restart the entire Replit instance

### If styles.css is missing:
The updated HTML references `styles.css`. Ensure this file exists in `holding/marketing-site/`. If needed, create a basic one:

```css
/* holding/marketing-site/styles.css */
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  line-height: 1.6;
  color: #333;
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  background: #f5f5f5;
}

.hero {
  text-align: center;
  padding: 60px 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 10px;
  margin-bottom: 40px;
}

.logo {
  font-size: 3em;
  font-weight: bold;
  margin-bottom: 10px;
}

.tag {
  font-size: 1.5em;
  margin-bottom: 10px;
}

.subtitle {
  font-size: 1.1em;
  opacity: 0.9;
  margin: 20px 0 10px 0;
}

.copyright {
  font-size: 0.9em;
  opacity: 0.7;
  margin-top: 20px;
}

.container {
  max-width: 1200px;
}

.card {
  background: white;
  padding: 30px;
  margin-bottom: 30px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.card.highlight {
  border-left: 4px solid #667eea;
}

.card.small {
  padding: 20px;
  background: #f9f9f9;
}

.card h2 {
  color: #667eea;
  margin-top: 0;
}

.lead {
  font-size: 1.1em;
  color: #555;
  margin-bottom: 20px;
}

.use-cases {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.use-case {
  padding: 20px;
  background: #f9f9f9;
  border-radius: 6px;
  border-left: 3px solid #667eea;
}

.benefits {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.benefit {
  padding: 20px;
  background: #f9f9f9;
  border-radius: 6px;
}

.benefit h4 {
  color: #667eea;
  margin-top: 0;
}

.metrics {
  width: 100%;
  border-collapse: collapse;
  margin: 20px 0;
}

.metrics th {
  background: #667eea;
  color: white;
  padding: 12px;
  text-align: left;
}

.metrics td {
  padding: 12px;
  border-bottom: 1px solid #ddd;
}

.metrics tr:hover {
  background: #f5f5f5;
}

pre {
  background: #2d2d2d;
  color: #f8f8f2;
  padding: 20px;
  border-radius: 6px;
  overflow-x: auto;
}

code {
  font-family: 'Courier New', monospace;
  font-size: 0.9em;
}

.resources {
  list-style: none;
  padding: 0;
}

.resources li {
  padding: 10px 0;
  border-bottom: 1px solid #eee;
}

.resources li:last-child {
  border-bottom: none;
}

.cta {
  display: inline-block;
  background: #667eea;
  color: white;
  padding: 12px 30px;
  border-radius: 6px;
  text-decoration: none;
  margin-top: 20px;
  font-weight: bold;
}

.cta:hover {
  background: #5568d3;
}

.footer {
  text-align: center;
  padding: 40px 20px;
  color: #666;
  border-top: 1px solid #ddd;
  margin-top: 40px;
}

.footer a {
  color: #667eea;
  text-decoration: none;
}

.footer a:hover {
  text-decoration: underline;
}

a {
  color: #667eea;
}

a:hover {
  color: #5568d3;
}
```

---

## Expected Result
After deployment, the website at https://e-rock-compute-eric589.replit.app/ should:
- Match the professional positioning of the GitHub README
- Show current performance metrics (GPU: 72.7M ops/sec, CPU: 1.6ms/100K)
- Present use cases for Industrial, AI/ML, Autonomous, and Data Center applications
- Include enterprise partnership and NDA program information
- Provide clear CTAs for commercial inquiries

---

## Verification Checklist
- [ ] Git pull completed successfully
- [ ] `index.html` contains "72.7M ops/sec" (not "193k ops/sec")
- [ ] `index.html` contains "Ultra-Efficient Mathematical Computation" in title
- [ ] Static file server is running and serving from correct directory
- [ ] Website loads at https://e-rock-compute-eric589.replit.app/
- [ ] All sections render correctly (use cases, metrics tables, platform support)
- [ ] Links to GitHub repository work
- [ ] CSS styling is applied (not plain HTML)
- [ ] Mobile responsive (test on narrow viewport)

---

## Notes
- The updated HTML is designed to work with the existing CSS structure
- If you see layout issues, verify `styles.css` exists and is properly linked
- The new content is longer (~400 lines) vs. old (~80 lines) - this is intentional
- All links point to the GitHub repository for detailed documentation
- Contact email is e@ewaller.com (verify this is correct)

---

**Priority:** HIGH  
**Estimated Time:** 5-10 minutes  
**Risk:** LOW (static HTML update, no backend changes)
