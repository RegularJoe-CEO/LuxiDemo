# Replit Agent: Website Update Instructions

## Objective
Update the Luxi Edge marketing website (https://e-rock-compute-eric589.replit.app/) to align with the newly redesigned GitHub landing page.

---

## Context
The GitHub repository README has been completely rewritten to present Luxi Edge as a professional, enterprise-ready product. The marketing website needs to match this new positioning.

**GitHub Repository:** https://github.com/RegularJoe-CEO/LuxiEdge  
**Current Website:** https://e-rock-compute-eric589.replit.app/

---

## Tasks

### 1. Pull Latest Changes from GitHub
```bash
git pull origin main
```

This will bring in the updated `holding/marketing-site/index.html` file (commit 672dfde).

### 2. Verify Updated Content
The new `index.html` should include:
- **New title:** "Luxi Edge — Ultra-Efficient Mathematical Computation"
- **Updated tagline:** "Energy-first compute platform delivering >10× performance and >5× energy efficiency"
- **Use case sections:** Industrial Control, AI/ML Pipelines, Autonomous Systems, Energy-Critical Infrastructure
- **Updated metrics:**
  - GPU: 72.7M ops/sec (NVIDIA L4)
  - CPU: 1.6ms for 100K elements
  - Platform support matrix (x86_64, ARM64, NVIDIA GPU)
- **Enterprise section:** White-label licensing, NDA partner program
- **Resources section:** Links to GitHub, docs, benchmarks, xAI guide

### 3. Deploy Updated Website
Ensure the Replit app serves the updated `index.html` file from `holding/marketing-site/`.

If the site uses a static file server, restart it:
```bash
# Example commands (adjust based on your Replit setup)
killall python3  # or node, or whatever serves the site
python3 -m http.server 8080 --directory holding/marketing-site &
```

Or if using Node.js:
```bash
cd holding/marketing-site
npx serve -p 8080
```

### 4. Verify Deployment
Visit https://e-rock-compute-eric589.replit.app/ and confirm:
- ✅ Page title is "Luxi Edge — Ultra-Efficient Mathematical Computation"
- ✅ GPU metrics show "72.7M ops/sec"
- ✅ Platform support table is present
- ✅ Enterprise partnership section is visible
- ✅ Links to GitHub repository work correctly

---

## Key Changes Summary

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
