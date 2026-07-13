# Publishing luxiedge.com on Replit (keep GoDaddy domain)

You already host on **Replit** and point **luxiedge.com** (GoDaddy DNS) at that Replit app.  
You do **not** need a new host. Replace the files in the **same** Replit project and redeploy.

## Mental model

```
Visitor → luxiedge.com (GoDaddy DNS)
       → Replit deployment (the live app)
       → static files (index.html, etc.)
```

GoDaddy only “redirects” / points DNS. **Replit serves the website.**  
To change what people see: change files in Replit, then publish/redeploy.

---

## Step 1 — Open the existing luxiedge Replit

1. Log into [replit.com](https://replit.com)
2. Open the project that currently powers **luxiedge.com**  
   (same one the Agent edits today — do **not** create a new Repl unless the old one is lost)

---

## Step 2 — Put the new site files in that Repl

You need the contents of local folder:

`~/LuxiDemo/site/`

| File / folder | Role |
|---------------|------|
| `index.html` | Home |
| `product.html` | Product |
| `data-centers.html` | Buyers |
| `energy.html` | Public energy story |
| `evidence.html` | Evidence |
| `benchmarks.html` | Benchmarks |
| `download.html` | Download |
| `contact.html` | Contact |
| `assets/site.css` | Styles |

**Do not** leave the old multi-page quant-risk site files as the homepage.  
Replace them so the Repl’s web root serves these pages.

### Option A — Upload zip (easiest, no Agent)

A ready zip is on your Mac Desktop:

**`~/Desktop/luxiedge-site-for-replit.zip`**

1. In Replit file tree: delete or move aside the **old** homepage HTML/CSS (keep a backup folder like `old-site-backup/` if you want)
2. Upload the zip (drag onto the file panel, or **⋯ → Upload file**)
3. Unzip so that **`index.html` is at the project root** (or in whatever folder Replit already serves as the web root — often the root)
4. Confirm structure looks like:

```text
your-repl/
  index.html
  product.html
  energy.html
  ...
  assets/
    site.css
```

### Option B — Tell the Agent (if you prefer chat)

Paste something like:

> Replace the website with the new static multi-page site.  
> Root must serve `index.html`. Keep pages: product, data-centers, energy, evidence, benchmarks, download, contact, and `assets/site.css`.  
> Do not invent new metrics. Do not restore the old quant-risk homepage.  
> After files are in place, configure the Repl to serve static HTML and redeploy.

Then upload or paste the files from `~/LuxiDemo/site/` (or the Desktop zip).

### Option C — Git import

If this Repl is linked to `RegularJoe-CEO/LuxiDemo`:

1. Push the local rebuild to GitHub first  
2. Pull in Replit  
3. Set the **run/serve root** to the `site/` folder (see Step 3)

---

## Step 3 — Make Replit serve static files

In the Replit **Shell** (or set as the Run command):

```bash
# If site files are at the project root:
python3 -m http.server 5000

# If you kept them in a site/ subfolder:
cd site && python3 -m http.server 5000
```

Notes:

- Replit often expects the process on **port 5000** (or whatever the Deployment panel shows). Match that port.
- If the old app used Node/Express, either:
  - switch Run to the Python static server above, **or**
  - point Express `static` middleware at the new files (Agent can do this if you already have a server).

In `.replit`, the run line should match. Example:

```toml
run = "python3 -m http.server 5000"
```

(Exact `.replit` format can vary; use the **Run** / **Deploy** UI if unsure.)

---

## Step 4 — Deploy / Publish (same as before)

1. Click **Deploy** or **Publish** (wording varies) on that Repl  
2. Use the **same** deployment that already has **luxiedge.com** attached  
3. Wait until the deployment is live  
4. Open the Replit URL first (`*.replit.app`) to confirm the **new** homepage  
5. Then open **https://luxiedge.com** (hard refresh / private window)

**GoDaddy:** if DNS already points at this Repl, you usually **change nothing**.  
Only touch GoDaddy if the domain was never connected or you created a brand-new Repl by mistake.

---

## Step 5 — Quick checks

| Check | Expect |
|-------|--------|
| Homepage H1 | “AI that uses less electricity — and proves it.” |
| Look | Light background, green accent (not neon-on-black) |
| `/energy.html` | “Ask your AI provider” |
| `/benchmarks.html` | Tables, no “label carefully” monologue |
| Old quant-risk hero | Gone |

---

## If something goes wrong

| Problem | Fix |
|---------|-----|
| Still see old site | Wrong Repl, or didn’t redeploy, or browser cache (private window) |
| 404 on product/energy | Files not at web root; `assets/` missing |
| Blank CSS | `assets/site.css` path wrong (must be `assets/site.css` next to HTML) |
| Domain broken after new Repl | Don’t make a new Repl — put files in the **existing** one that owns the domain |
| Agent rewrites copy / metrics | Stop it; upload zip yourself (Option A) |

---

## Local preview (unchanged)

```bash
cd ~/LuxiDemo/site && python3 -m http.server 8877 --bind 127.0.0.1
# http://127.0.0.1:8877/
```

---

## Summary

1. Open **existing** luxiedge Repl  
2. Upload **`Desktop/luxiedge-site-for-replit.zip`** (or copy `~/LuxiDemo/site/*`)  
3. Serve with static server on Replit’s port  
4. **Redeploy** the same deployment  
5. Leave GoDaddy alone if domain already works  

Contact: e@ewaller.com
