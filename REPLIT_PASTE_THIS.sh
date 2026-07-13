#!/usr/bin/env bash
set -euo pipefail
cd "$HOME/workspace" 2>/dev/null || cd /home/runner/workspace 2>/dev/null || cd "$(pwd)"
echo "Installing LuxiEdge site into: $(pwd)"
mkdir -p assets static/assets old-site-backup
echo "Writing index.html..."
cat > 'index.html' << '__LUXI_index_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>LuxiEdge — AI that uses less electricity</title>
  <meta name="description" content="LuxiEdge: energy-aware AI for quant research, data centers, and the public. Measured savings. Share the story. Demand better from AI providers.">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Space+Grotesk:wght@500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="assets/site.css">
</head>
<body>
  <header class="site-header">
    <div class="wrap nav">
      <a class="brand" href="index.html">Lu(x)i<span>Edge</span></a>
      <ul class="nav-links">
        <li><a href="quant-research.html">Quant &amp; research</a></li>
        <li><a href="data-centers.html">AI &amp; data centers</a></li>
        <li><a href="help.html">We need your help</a></li>
        <li><a href="proof.html">Proof</a></li>
        <li><a class="cta-nav" href="contact.html">Contact</a></li>
      </ul>
    </div>
  </header>

  <main>
    <div class="hero">
      <div class="wrap">
        <p class="eyebrow">Three audiences · one mission</p>
        <h1>AI that uses less electricity — and proves it.</h1>
        <p class="lede">
          LuxiEdge is built for <strong>quant research</strong>, <strong>AI data centers</strong>,
          and a <strong>public that can drive change</strong> by sharing the energy story
          and asking providers why they are not measuring and cutting waste.
        </p>
        <div class="cta-row">
          <a class="btn btn-primary" href="help.html">We need your help →</a>
          <a class="btn btn-secondary" href="data-centers.html">AI &amp; data centers</a>
          <a class="btn btn-ghost" href="quant-research.html">Quant &amp; research</a>
        </div>
        <div class="metrics">
          <div class="metric">
            <div class="val">~403</div>
            <div class="label">Tokens per second</div>
            <div class="src"><a href="proof.html#benchmarks">Public measurement</a></div>
          </div>
          <div class="metric">
            <div class="val">0.63</div>
            <div class="label">Joules per token</div>
            <div class="src"><a href="proof.html#benchmarks">Under sustained load</a></div>
          </div>
          <div class="metric">
            <div class="val">~254 W</div>
            <div class="label">Power during the work</div>
            <div class="src">Full 28-layer stack</div>
          </div>
          <div class="metric">
            <div class="val">O(N)</div>
            <div class="label">Long-context memory scaling</div>
            <div class="src"><a href="proof.html#benchmarks">vs dense O(N²)</a></div>
          </div>
        </div>
        <p class="method">
          Numbers from public multi-run benchmarks.
          Full methods and downloads: <a href="proof.html">Proof</a>.
        </p>
      </div>
    </div>

    <section>
      <div class="wrap">
        <h2>A three-legged stool</h2>
        <p class="sub">Pick the path that fits you. Every path can end in shareable proof.</p>
        <div class="two-path" style="grid-template-columns: 1fr 1fr 1fr;">
          <div class="card">
            <h3>Quant &amp; research</h3>
            <p>Determinism, audit trails, free-ride paths, long-context memory — for people who need methods, not slogans.</p>
            <p style="margin-top:1rem"><a class="btn btn-secondary" href="quant-research.html">Science path →</a></p>
          </div>
          <div class="card accent">
            <h3>AI &amp; data centers</h3>
            <p>Power caps, density, measured joules-per-token under load, and a commercial path to evaluation.</p>
            <p style="margin-top:1rem"><a class="btn btn-primary" href="data-centers.html">Operator path →</a></p>
          </div>
          <div class="card">
            <h3>We need your help</h3>
            <p>You don’t need a PhD. Share the energy story. Ask AI providers and data centers why they aren’t cutting waste.</p>
            <p style="margin-top:1rem"><a class="btn btn-secondary" href="help.html">Public path →</a></p>
          </div>
        </div>
      </div>
    </section>

    <section>
      <div class="wrap">
        <h2>What we measure — honestly</h2>
        <div class="grid-2">
          <div class="card">
            <h3>On the public record</h3>
            <ul>
              <li>Full <strong>28-layer 7B-class</strong> stack on H100</li>
              <li>Multi-run throughput and energy per token</li>
              <li>Supporting stack energy and free-ride tests</li>
              <li>Long-context memory scaling (O(N) vs O(N²))</li>
            </ul>
          </div>
          <div class="card">
            <h3>What we do not claim</h3>
            <ul>
              <li>Not a claim of chat-quality superiority over every model</li>
              <li>Some standard stacks still win raw short-sequence speed</li>
              <li>We differentiate on audit, memory scaling, and measured energy</li>
              <li>Facility wall-plug power needs separate metering</li>
            </ul>
          </div>
        </div>
        <div class="callout">
          <strong>We publish comparisons both ways.</strong>
          See the <a href="proof.html">Proof</a> page for packs, tables, and downloads.
        </div>
      </div>
    </section>

    <section>
      <div class="wrap">
        <h2>Ready to act?</h2>
        <div class="cta-row">
          <a class="btn btn-primary" href="help.html">Share the energy story</a>
          <a class="btn btn-secondary" href="proof.html">See the proof</a>
          <a class="btn btn-ghost" href="contact.html">Contact</a>
        </div>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="wrap footer-grid">
      <div>
        <strong style="color:var(--text)">LuxiEdge</strong><br>
        Eric Waller · <a href="mailto:e@ewaller.com">e@ewaller.com</a>
      </div>
      <div>
        Public proof:
        <a href="https://github.com/RegularJoe-CEO/LuxiDemo">github.com/RegularJoe-CEO/LuxiDemo</a>
      </div>
    </div>
  </footer>
</body>
</html>
__LUXI_index_html__

echo "Writing quant-research.html..."
cat > 'quant-research.html' << '__LUXI_quant-research_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Quant & research — LuxiEdge</title>
  <meta name="description" content="LuxiEdge for quant and research: TRADE stacks, AUDIT/receipts, WNSM free-ride, O(N) long-context memory, convert+serve.">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Space+Grotesk:wght@500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="assets/site.css">
</head>
<body>
  <header class="site-header">
    <div class="wrap nav">
      <a class="brand" href="index.html">Lu(x)i<span>Edge</span></a>
            <ul class="nav-links">
        <li><a href="quant-research.html" aria-current="page">Quant &amp; research</a></li>
        <li><a href="data-centers.html">AI &amp; data centers</a></li>
        <li><a href="help.html">We need your help</a></li>
        <li><a href="proof.html">Proof</a></li>
        <li><a class="cta-nav" href="contact.html">Contact</a></li>
      </ul>
    </div>
  </header>

  <main>
    <div class="page-hero">
      <div class="wrap">
        <p class="eyebrow">Quant &amp; research</p>
        <h1>Auditable attention paths. Measured energy. Public packs.</h1>
        <p class="lede">
          Infrastructure for deterministic, receipt-attested, energy-aware AI compute —
          with numbers you can open on GitHub.
        </p>
      </div>
    </div>

    <section>
      <div class="wrap">
        <h2>Core capabilities</h2>
        <div class="grid-2">
          <div class="card">
            <h3>TRADE residual stacks</h3>
            <p>Device-resident GPU stacks for multi-layer residual compute. Public 7B-class path: <strong style="color:var(--text)">28 layers × h=3584</strong> on H100 NVL with multi-run thr and sustain-only J/token.</p>
          </div>
          <div class="card">
            <h3>AUDIT &amp; receipts</h3>
            <p>Where the product lane demands it: cryptographic receipts, free-ride residual checks, and null-space behavior under load — not just marketing “determinism.”</p>
          </div>
          <div class="card">
            <h3>WNSM free-ride</h3>
            <p>Null-space payload bus under real CUDA load. Public pack: free-ride vs side-channel H2D, null residual ~1e-8 class, single-layer null-inject drift 0 in the stack test path.</p>
          </div>
          <div class="card">
            <h3>Long-context O(N) memory</h3>
            <p>Waller streaming state scales ~O(N) in memory vs dense scores ~O(N²). Public ladder through 32k (and analytical 131k memory reduction).</p>
          </div>
          <div class="card">
            <h3>Convert + serve path</h3>
            <p>HF → Luxi native convert for 7B-class weights; serve and TRADE examples for operator evaluation. Implementation lives in the engineering repo; <strong style="color:var(--text)">proof is public on LuxiDemo</strong>.</p>
          </div>
          <div class="card">
            <h3>Deterministic math demo</h3>
            <p>Standalone binary: JSON expressions in, results + SHA256 out. Useful for integration smoke tests — secondary to the inference-energy thesis on this site.</p>
          </div>
        </div>
      </div>
    </section>

    <section>
      <div class="wrap">
        <h2>Where we win vs where we are honest</h2>
        <div class="table-wrap">
          <table>
            <thead>
              <tr><th>Axis</th><th>Status</th><th>Public proof</th></tr>
            </thead>
            <tbody>
              <tr>
                <td>7B-class board J/token @ seq≥128</td>
                <td>~0.63 J/tok · ~403 tok/s</td>
                <td><a href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-7b-class-TRADE">h100-7b-class-TRADE</a></td>
              </tr>
              <tr>
                <td>Short-seq thr vs Flash</td>
                <td>Flash-class may win (published H2H loss)</td>
                <td><a href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-stack12-H2H">h100-stack12-H2H</a></td>
              </tr>
              <tr>
                <td>Free-ride / AUDIT under load</td>
                <td>Differentiated thesis</td>
                <td><a href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-WNSM-free-ride">h100-WNSM-free-ride</a></td>
              </tr>
              <tr>
                <td>Long-ctx memory scaling</td>
                <td>O(N) vs O(N²)</td>
                <td><a href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-LONGCTX-scaling">h100-LONGCTX-scaling</a></td>
              </tr>
            </tbody>
          </table>
        </div>
        <div class="callout">
          <strong>Not HF chat SOTA.</strong> The 7B-class TRADE path maps architecture into TRADE kernels (e.g. GQA/SwiGLU into the GELU-MHA TRADE path). Use it for energy / stack evaluation, not as a drop-in chat quality claim.
        </div>
      </div>
    </section>

    <section>
      <div class="wrap">
        <h2>Next steps</h2>
        <div class="cta-row">
          <a class="btn btn-primary" href="proof.html#benchmarks">See the proof</a>
          <a class="btn btn-secondary" href="data-centers.html">Data-center buyer page</a>
          <a class="btn btn-ghost" href="contact.html">Talk to us</a>
        </div>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="wrap footer-grid">
      <div>Eric Waller · <a href="mailto:e@ewaller.com">e@ewaller.com</a></div>
      <div><a href="https://github.com/RegularJoe-CEO/LuxiDemo">Public LuxiDemo</a></div>
    </div>
  </footer>
</body>
</html>
__LUXI_quant-research_html__

echo "Writing data-centers.html..."
cat > 'data-centers.html' << '__LUXI_data-centers_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>AI & data centers — LuxiEdge</title>
  <meta name="description" content="For data-center and quant AI buyers: power, density, auditability, measured J/token on 7B-class stacks.">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Space+Grotesk:wght@500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="assets/site.css">
</head>
<body>
  <header class="site-header">
    <div class="wrap nav">
      <a class="brand" href="index.html">Lu(x)i<span>Edge</span></a>
            <ul class="nav-links">
        <li><a href="quant-research.html">Quant &amp; research</a></li>
        <li><a href="data-centers.html" aria-current="page">AI &amp; data centers</a></li>
        <li><a href="help.html">We need your help</a></li>
        <li><a href="proof.html">Proof</a></li>
        <li><a class="cta-nav" href="contact.html">Contact</a></li>
      </ul>
    </div>
  </header>

  <main>
    <div class="page-hero">
      <div class="wrap">
        <p class="eyebrow">AI &amp; data centers</p>
        <h1>Power is the product constraint. Audit is the trust constraint.</h1>
        <p class="lede">
          If you run multi-tenant inference, regulated quant AI, or capacity planning under power caps,
          you need thr <em>and</em> joules-per-token under load — with methods you can reproduce.
        </p>
      </div>
    </div>

    <section>
      <div class="wrap">
        <h2>The problem we speak to</h2>
        <div class="grid-2">
          <div class="card">
            <h3>Power &amp; density</h3>
            <p>GPUs that look idle on a dashboard can still be a facilities problem when stacks are wrong — or a wasted power budget when metrics are silicon TFLOPS instead of workload J/token.</p>
          </div>
          <div class="card">
            <h3>Trust &amp; audit</h3>
            <p>Regulated desks and multi-tenant platforms need reproducibility stories that survive re-runs, not “it was about the same.”</p>
          </div>
        </div>
      </div>
    </section>

    <section>
      <div class="wrap">
        <h2>What we put in front of diligence</h2>
        <div class="metrics">
          <div class="metric">
            <div class="val">~403 tok/s</div>
            <div class="label">7B-class prefill · seq=128</div>
          </div>
          <div class="metric">
            <div class="val">0.63 J/tok</div>
            <div class="label">Board energy · multi-run</div>
          </div>
          <div class="metric">
            <div class="val">~254 W</div>
            <div class="label">Median board power</div>
          </div>
          <div class="metric">
            <div class="val">28L</div>
            <div class="label">Full stack · h=3584</div>
          </div>
        </div>
        <p class="method">
          Protocol: H100 NVL · full stack under sustained load · energy from measured power × time / tokens.
          <a href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-7b-class-TRADE">Open the pack →</a>
        </p>
      </div>
    </section>

    <section>
      <div class="wrap">
        <h2>Operator pillars</h2>
        <div class="grid-2">
          <div class="card accent">
            <h3>1. Measure under load</h3>
            <p>Multi-run sustains at realistic sequence lengths. Publish head-to-head results even when a baseline wins.</p>
          </div>
          <div class="card accent">
            <h3>2. Memory that scales</h3>
            <p>O(N) streaming-state memory vs dense O(N²) score matrices — critical when context length is the silent capacity killer.</p>
          </div>
          <div class="card">
            <h3>3. Free-ride / AUDIT axes</h3>
            <p>WNSM null-space payload under CUDA load. Residual checks. Differentiated from pure thr leaderboards.</p>
          </div>
          <div class="card">
            <h3>4. Public first, NDA second</h3>
            <p>Every headline on this site links a public GitHub pack. Full source / commercial deployment under NDA when you are ready.</p>
          </div>
        </div>
      </div>
    </section>

    <section>
      <div class="wrap">
        <h2>Integration sketch</h2>
        <ol style="color:var(--muted); padding-left:1.25rem; max-width:60ch">
          <li style="margin:0.4rem 0">Review public packs on LuxiDemo (evidence index).</li>
          <li style="margin:0.4rem 0">Run demo math binary for receipt smoke tests if useful.</li>
          <li style="margin:0.4rem 0">Engineering evaluation of convert + TRADE stack path under NDA / commercial terms.</li>
          <li style="margin:0.4rem 0">Joint power measurement plan on your hardware (board + optional facility meters).</li>
        </ol>
        <div class="cta-row" style="margin-top:1.5rem">
          <a class="btn btn-primary" href="contact.html">Request technical discussion</a>
          <a class="btn btn-secondary" href="proof.html">Proof</a>
        </div>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="wrap footer-grid">
      <div>Eric Waller · <a href="mailto:e@ewaller.com">e@ewaller.com</a></div>
      <div>No customer logos. No diligence codenames. Packs only.</div>
    </div>
  </footer>
</body>
</html>
__LUXI_data-centers_html__

echo "Writing help.html..."
cat > 'help.html' << '__LUXI_help_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>We need your help — LuxiEdge</title>
  <meta name="description" content="AI burns enormous electricity. LuxiEdge measures energy under load. Share the story. Ask data centers and AI providers why they are not cutting waste.">
  <meta property="og:title" content="We need your help — AI that uses less electricity">
  <meta property="og:description" content="Data centers powering AI are among the fastest-growing electricity users on Earth. LuxiEdge measures energy under load. Share this and ask your provider why they are not using it.">
  <meta property="og:type" content="website">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Space+Grotesk:wght@500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="assets/site.css">
</head>
<body>
  <header class="site-header">
    <div class="wrap nav">
      <a class="brand" href="index.html">Lu(x)i<span>Edge</span></a>
      <ul class="nav-links">
        <li><a href="quant-research.html">Quant &amp; research</a></li>
        <li><a href="data-centers.html">AI &amp; data centers</a></li>
        <li><a href="help.html" aria-current="page">We need your help</a></li>
        <li><a href="proof.html">Proof</a></li>
        <li><a class="cta-nav" href="contact.html">Contact</a></li>
      </ul>
    </div>
  </header>

  <main>
    <div class="page-hero">
      <div class="wrap">
        <p class="eyebrow">Public call to action</p>
        <h1>We need your help to cut the electricity waste in AI.</h1>
        <p class="lede">
          You do not need to be an engineer. You need to care that every chat, every model, every data center
          burns real power — and that <strong>measured savings</strong> only win when people demand them.
          LuxiEdge publishes the numbers. <strong>You spread the story.</strong>
        </p>
        <div class="cta-row" style="margin-top:1.25rem">
          <a class="btn btn-primary" href="#share">Share this now</a>
          <a class="btn btn-secondary" href="#ask">Ask your AI provider</a>
          <a class="btn btn-ghost" href="proof.html">See the proof</a>
        </div>
      </div>
    </div>

    <section>
      <div class="wrap">
        <h2>The simple story</h2>
        <p class="sub">
          AI data centers are among the fastest-growing electricity users on the planet.
          Heat and cooling make the bill even larger. If a company claims “efficiency,”
          they should show <strong>energy per unit of work</strong> — not just pretty charts.
        </p>
        <div class="metrics">
          <div class="metric">
            <div class="val">0.63 J</div>
            <div class="label">Energy per token of work</div>
            <div class="src">Public multi-run measurement</div>
          </div>
          <div class="metric">
            <div class="val">~400</div>
            <div class="label">Tokens per second</div>
            <div class="src">Same measured runs</div>
          </div>
          <div class="metric">
            <div class="val">Public</div>
            <div class="label">Anyone can open the traces</div>
            <div class="src"><a href="proof.html">Proof page</a></div>
          </div>
        </div>
        <div class="callout" style="margin-top:1.5rem">
          <strong>Less energy per unit of work means lower electricity cost</strong> for the same amount of AI —
          and less heat that has to be cooled. That is the savings story.
        </div>
      </div>
    </section>

    <section id="share">
      <div class="wrap">
        <h2>Share this — that is the help we need</h2>
        <p class="sub">
          Public pressure works when the same question shows up in feeds, inboxes, and boardrooms.
          One share is a vote for measured efficiency.
        </p>
        <div class="cta-row">
          <a class="btn btn-primary" target="_blank" rel="noopener"
             href="https://twitter.com/intent/tweet?text=AI%20data%20centers%20burn%20enormous%20electricity.%20LuxiEdge%20measures%20energy%20under%20load%20and%20publishes%20the%20numbers.%20Ask%20your%20provider%3A%20why%20aren%27t%20you%20using%20this%3F&url=https%3A%2F%2Fluxiedge.com%2Fhelp.html">
            Share on X
          </a>
          <a class="btn btn-secondary" target="_blank" rel="noopener"
             href="https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fluxiedge.com%2Fhelp.html">
            Share on LinkedIn
          </a>
          <a class="btn btn-ghost"
             href="mailto:?subject=AI%20energy%20%E2%80%94%20worth%20a%20look&body=AI%20uses%20a%20huge%20amount%20of%20electricity.%20LuxiEdge%20publishes%20measured%20energy%20under%20load%20and%20asks%20providers%20to%20do%20better.%0A%0Ahttps%3A%2F%2Fluxiedge.com%2Fhelp.html%0A%0AProof%3A%20https%3A%2F%2Fluxiedge.com%2Fproof.html">
            Email a friend
          </a>
        </div>
        <p class="method" style="margin-top:1rem">
          Suggested post text is already filled in. Edit freely — keep the link so people can verify.
        </p>
      </div>
    </section>

    <section id="ask">
      <div class="wrap">
        <h2>Ask your AI provider and data-center operators</h2>
        <p class="sub">Copy these questions. Paste into support, social, or email:</p>
        <ol style="color:var(--muted); padding-left:1.25rem; max-width:58ch">
          <li style="margin:0.55rem 0">
            <strong>What is your energy cost per token</strong> (or per million tokens) under sustained load?
          </li>
          <li style="margin:0.55rem 0">
            <strong>Where are the public numbers?</strong> Can a stranger download the method?
          </li>
          <li style="margin:0.55rem 0">
            <strong>Have you evaluated LuxiEdge?</strong> If not, why not — and when will you?
          </li>
          <li style="margin:0.55rem 0">
            <strong>Is energy per unit of work going down</strong> year over year at your facilities?
          </li>
        </ol>
        <div class="cta-row" style="margin-top:1.5rem">
          <a class="btn btn-primary"
             href="mailto:?subject=AI%20energy%20efficiency%20%E2%80%94%20why%20not%20LuxiEdge%3F&body=I%20want%20to%20know%20your%20energy%20per%20token%20under%20sustained%20load%2C%20and%20whether%20you%20have%20evaluated%20LuxiEdge%20(https%3A%2F%2Fluxiedge.com).%0A%0APublic%20measurements%3A%20https%3A%2F%2Fluxiedge.com%2Fproof.html">
            Email your provider a draft
          </a>
          <a class="btn btn-secondary" href="data-centers.html">If you run a data center →</a>
        </div>
        <div class="callout" style="margin-top:1.5rem">
          <strong>The goal is simple:</strong> make waste expensive to ignore.
          When customers, voters, and journalists ask the same questions, operators move.
        </div>
      </div>
    </section>

    <section>
      <div class="wrap">
        <h2>Why this is not just marketing noise</h2>
        <div class="grid-2">
          <div class="card">
            <h3>Your power bill is connected</h3>
            <p>
              When AI demand spikes, utilities build capacity and raise rates.
              Communities near large data centers feel water, land, and grid pressure first.
              Efficiency is a public interest.
            </p>
          </div>
          <div class="card">
            <h3>Slogans are not enough</h3>
            <p>
              “We care about the climate” without published energy-per-work numbers is empty.
              Demand measurements under real load. Point people at
              <a href="proof.html">Proof</a>.
            </p>
          </div>
        </div>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="wrap footer-grid">
      <div>Eric Waller · <a href="mailto:e@ewaller.com">e@ewaller.com</a></div>
      <div>Share the story. Demand the numbers.</div>
    </div>
  </footer>
</body>
</html>
__LUXI_help_html__

echo "Writing proof.html..."
cat > 'proof.html' << '__LUXI_proof_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Proof — LuxiEdge</title>
  <meta name="description" content="Public evidence packs, benchmarks, and demo downloads for LuxiEdge.">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Space+Grotesk:wght@500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="assets/site.css">
</head>
<body>
  <header class="site-header">
    <div class="wrap nav">
      <a class="brand" href="index.html">Lu(x)i<span>Edge</span></a>
      <ul class="nav-links">
        <li><a href="quant-research.html">Quant &amp; research</a></li>
        <li><a href="data-centers.html">AI &amp; data centers</a></li>
        <li><a href="help.html">We need your help</a></li>
        <li><a href="proof.html" aria-current="page">Proof</a></li>
        <li><a class="cta-nav" href="contact.html">Contact</a></li>
      </ul>
    </div>
  </header>

  <main>
    <div class="page-hero">
      <div class="wrap">
        <p class="eyebrow">Evidence · benchmarks · download</p>
        <h1>Proof you can open — no NDA required for the headlines.</h1>
        <p class="lede">
          Every material number on this site points here: public packs, measured tables, and demo binaries.
        </p>
        <div class="cta-row" style="margin-top:1rem">
          <a class="btn btn-primary" href="#benchmarks">Benchmarks</a>
          <a class="btn btn-secondary" href="#packs">Evidence packs</a>
          <a class="btn btn-ghost" href="#download">Download</a>
        </div>
      </div>
    </div>

    <section id="benchmarks">
      <div class="wrap">
        <h2>Benchmarks</h2>
        <p class="sub">
          Results from public LuxiDemo evidence on H100 hardware (2026-07-11).
          Open any pack to inspect methods and raw traces.
        </p>
        <h3 style="font-family:var(--display);color:var(--text);margin:1.25rem 0 0.5rem;font-size:1.1rem">7B-class TRADE — energy &amp; throughput</h3>
        <p class="sub" style="margin-bottom:0.75rem">
          Hardware: 1× NVIDIA H100 NVL · 28-layer stack ·
          <a href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-7b-class-TRADE">public pack</a>.
        </p>
        <div class="table-wrap">
          <table>
            <thead>
              <tr>
                <th>Sequence length</th>
                <th>Tokens / sec</th>
                <th>Joules / token</th>
                <th>Median power (W)</th>
                <th>Notes</th>
              </tr>
            </thead>
            <tbody>
              <tr><td>5</td><td>~44</td><td>3.560 ± 0.005</td><td>~156</td><td>Short-burst reference</td></tr>
              <tr><td>32</td><td>~221</td><td>—</td><td>—</td><td>Throughput sweep</td></tr>
              <tr><td>64</td><td>~359</td><td>—</td><td>—</td><td>Throughput sweep</td></tr>
              <tr class="row-highlight"><td>128</td><td>~403</td><td>0.630 ± 0.002</td><td>~254</td><td>Primary result</td></tr>
              <tr><td>256</td><td>~464</td><td>~0.604</td><td>~280</td><td>Longer context</td></tr>
              <tr><td>512</td><td>~493</td><td>—</td><td>—</td><td>Throughput sweep</td></tr>
            </tbody>
          </table>
        </div>
        <p class="method">Primary multi-run at sequence length 128. Source: <code>evidence/h100-7b-class-TRADE/</code></p>

        <h3 style="font-family:var(--display);color:var(--text);margin:2rem 0 0.5rem;font-size:1.1rem">Head-to-head with a Flash baseline</h3>
        <p class="sub">12 layers · sequence 1024 · h=768 · ~10 s each side.</p>
        <div class="table-wrap">
          <table>
            <thead>
              <tr><th>Metric</th><th>TRADE 12L</th><th>PyTorch + Flash 12L</th><th>Ratio</th></tr>
            </thead>
            <tbody>
              <tr><td>Median power (W)</td><td>177.2</td><td>176.4</td><td>~1.0×</td></tr>
              <tr><td>ms / stack forward</td><td>74.32</td><td>3.90</td><td>TRADE 19.1× slower</td></tr>
              <tr><td>Prefill tok/s</td><td>13778</td><td>262859</td><td>PT 19.1× higher</td></tr>
              <tr><td>Joules / token</td><td>0.0129</td><td>0.0007</td><td>TRADE 19.2× higher</td></tr>
            </tbody>
          </table>
        </div>
        <p class="footnote">
          <a href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-stack12-H2H">h100-stack12-H2H</a> —
          we publish where a standard baseline wins on this shape.
        </p>

        <h3 style="font-family:var(--display);color:var(--text);margin:2rem 0 0.5rem;font-size:1.1rem">Long-context memory</h3>
        <div class="table-wrap">
          <table>
            <thead>
              <tr><th>Sequence length</th><th>Waller state (MB)</th><th>Dense scores (MB)</th><th>Reduction</th></tr>
            </thead>
            <tbody>
              <tr><td>1,024</td><td>0.52</td><td>8.4</td><td>16×</td></tr>
              <tr><td>4,096</td><td>2.1</td><td>134</td><td>64×</td></tr>
              <tr><td>8,192</td><td>4.2</td><td>537</td><td>128×</td></tr>
              <tr><td>32,768</td><td>16.8</td><td>4,295</td><td>256×</td></tr>
            </tbody>
          </table>
        </div>
        <p class="method">
          Memory scales near-linearly vs dense O(N²).
          <a href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-LONGCTX-scaling">Pack →</a>
        </p>
      </div>
    </section>

    <section id="packs">
      <div class="wrap">
        <h2>Evidence packs</h2>
        <p class="sub">Open on GitHub. Read methods. Check traces.</p>
        <a class="pack" href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-7b-class-TRADE">
          <span class="tag">Lead pack</span>
          <h3>h100-7b-class-TRADE</h3>
          <p>Full 28-layer 7B-class TRADE stack. Multi-run thr + energy (~0.63 J/tok @ seq=128, ~403 tok/s).</p>
        </a>
        <a class="pack" href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-stack12-TRADE-cuda">
          <span class="tag">Energy</span>
          <h3>h100-stack12-TRADE-cuda</h3>
          <p>Device-resident 12-layer stack energy (GPT-2 width).</p>
        </a>
        <a class="pack" href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-stack12-H2H">
          <span class="tag">Honesty</span>
          <h3>h100-stack12-H2H</h3>
          <p>TRADE vs PyTorch Flash — we publish the loss on this shape.</p>
        </a>
        <a class="pack" href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-WNSM-free-ride">
          <span class="tag">AUDIT</span>
          <h3>h100-WNSM-free-ride</h3>
          <p>Null-space free-ride under GPU load.</p>
        </a>
        <a class="pack" href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-LONGCTX-scaling">
          <span class="tag">Memory</span>
          <h3>h100-LONGCTX-scaling</h3>
          <p>O(N) vs O(N²) memory ladder + CUDA 32k.</p>
        </a>
        <a class="pack" href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-BASELINE-vs-geo">
          <span class="tag">Micro</span>
          <h3>h100-BASELINE-vs-geo</h3>
          <p>Single-layer baseline wedges — not a 7B thr claim.</p>
        </a>
        <a class="pack" href="https://github.com/RegularJoe-CEO/LuxiDemo/tree/main/evidence/h100-serve-sustain-2026-07-11">
          <span class="tag">Serve</span>
          <h3>h100-serve-sustain-2026-07-11</h3>
          <p>Continuous-batch serve sustain with power traces.</p>
        </a>
        <p style="margin-top:1rem">
          <a class="btn btn-secondary" href="https://github.com/RegularJoe-CEO/LuxiDemo">Open LuxiDemo on GitHub</a>
        </p>
      </div>
    </section>

    <section id="download">
      <div class="wrap">
        <h2>Download</h2>
        <p class="sub">
          Public demo builds for the expression/receipt path. Demo only — not production inference.
          Energy claims live in the packs above, not only inside these binaries.
        </p>
        <a class="btn btn-primary" href="https://github.com/RegularJoe-CEO/LuxiDemo/releases/latest">Open latest release →</a>
        <div class="grid-2" style="margin-top:1.5rem">
          <div class="card">
            <h3>LuxiEdge Lite / Demo</h3>
            <p>macOS ARM, Linux x86 (CPU/GPU), Linux ARM, Windows — from the public release channel.</p>
          </div>
          <div class="card">
            <h3>Quick start</h3>
<pre>chmod +x luxiedge-*-macos-arm64
./luxiedge-*-macos-arm64 --port 9090

curl -X POST http://localhost:9090/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"sin(x)","values":[0.5,1.0],"precision":"f32"}'</pre>
          </div>
        </div>
        <div class="callout">
          <strong>Demo binaries may expire.</strong> Full operator sets require a license —
          <a href="contact.html">contact</a>.
        </div>
        <div class="cta-row" style="margin-top:1.25rem">
          <a class="btn btn-secondary" href="help.html">We need your help</a>
          <a class="btn btn-ghost" href="data-centers.html">AI &amp; data centers</a>
        </div>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="wrap footer-grid">
      <div>Eric Waller · <a href="mailto:e@ewaller.com">e@ewaller.com</a></div>
      <div><a href="https://github.com/RegularJoe-CEO/LuxiDemo">github.com/RegularJoe-CEO/LuxiDemo</a></div>
    </div>
  </footer>
</body>
</html>
__LUXI_proof_html__

echo "Writing contact.html..."
cat > 'contact.html' << '__LUXI_contact_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Contact — LuxiEdge</title>
  <meta name="description" content="Contact Eric Waller for LuxiEdge commercial evaluation, NDA source review, and data-center technical discussion.">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Space+Grotesk:wght@500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="assets/site.css">
</head>
<body>
  <header class="site-header">
    <div class="wrap nav">
      <a class="brand" href="index.html">Lu(x)i<span>Edge</span></a>
            <ul class="nav-links">
        <li><a href="quant-research.html">Quant &amp; research</a></li>
        <li><a href="data-centers.html">AI &amp; data centers</a></li>
        <li><a href="help.html">We need your help</a></li>
        <li><a href="proof.html">Proof</a></li>
        <li><a class="cta-nav" href="contact.html" aria-current="page">Contact</a></li>
      </ul>
    </div>
  </header>

  <main>
    <div class="page-hero">
      <div class="wrap">
        <p class="eyebrow">Commercial · NDA · technical</p>
        <h1>Ready for a confidential technical discussion.</h1>
        <p class="lede">
          Public packs first. Then we talk deployment, source access, and joint measurement on your metal.
        </p>
      </div>
    </div>

    <section>
      <div class="wrap">
        <div class="grid-2">
          <div class="card accent">
            <h3>Email</h3>
            <p style="font-size:1.2rem; margin:0.75rem 0">
              <a href="mailto:e@ewaller.com?subject=LuxiEdge%20technical%20discussion">e@ewaller.com</a>
            </p>
            <p>Eric Waller · proprietary technology · full source under NDA when appropriate.</p>
          </div>
          <div class="card">
            <h3>Before you write</h3>
            <ul>
              <li>Skim <a href="proof.html">public proof</a></li>
              <li>Note hardware you care about (GPU model, power cap)</li>
              <li>Say whether you need thr, energy, audit, or all three</li>
            </ul>
          </div>
        </div>
        <div class="callout" style="margin-top:1.5rem">
          We do not publish customer or diligence-target names on this site. Your evaluation stays private.
        </div>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="wrap footer-grid">
      <div>Eric Waller · <a href="mailto:e@ewaller.com">e@ewaller.com</a></div>
      <div><a href="https://github.com/RegularJoe-CEO/LuxiDemo">Public proof on GitHub</a></div>
    </div>
  </footer>
</body>
</html>
__LUXI_contact_html__

echo "Writing product.html..."
cat > 'product.html' << '__LUXI_product_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; url=quant-research.html">
  <link rel="canonical" href="quant-research.html">
  <title>Moved — LuxiEdge</title>
  <script>location.replace("quant-research.html");</script>
</head>
<body>
  <p>This page moved to <a href="quant-research.html">quant-research.html</a>.</p>
</body>
</html>
__LUXI_product_html__

echo "Writing energy.html..."
cat > 'energy.html' << '__LUXI_energy_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; url=help.html">
  <link rel="canonical" href="help.html">
  <title>Moved — LuxiEdge</title>
  <script>location.replace("help.html");</script>
</head>
<body>
  <p>This page moved to <a href="help.html">help.html</a>.</p>
</body>
</html>
__LUXI_energy_html__

echo "Writing evidence.html..."
cat > 'evidence.html' << '__LUXI_evidence_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; url=proof.html">
  <link rel="canonical" href="proof.html">
  <title>Moved — LuxiEdge</title>
  <script>location.replace("proof.html");</script>
</head>
<body>
  <p>This page moved to <a href="proof.html">proof.html</a>.</p>
</body>
</html>
__LUXI_evidence_html__

echo "Writing benchmarks.html..."
cat > 'benchmarks.html' << '__LUXI_benchmarks_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; url=proof.html#benchmarks">
  <link rel="canonical" href="proof.html#benchmarks">
  <title>Moved — LuxiEdge</title>
  <script>location.replace("proof.html#benchmarks");</script>
</head>
<body>
  <p>This page moved to <a href="proof.html#benchmarks">proof.html#benchmarks</a>.</p>
</body>
</html>
__LUXI_benchmarks_html__

echo "Writing download.html..."
cat > 'download.html' << '__LUXI_download_html__'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; url=proof.html#download">
  <link rel="canonical" href="proof.html#download">
  <title>Moved — LuxiEdge</title>
  <script>location.replace("proof.html#download");</script>
</head>
<body>
  <p>This page moved to <a href="proof.html#download">proof.html#download</a>.</p>
</body>
</html>
__LUXI_download_html__

echo "Writing assets/site.css..."
mkdir -p "assets"
cat > 'assets/site.css' << '__LUXI_assets_site_css__'
/* LuxiEdge site — light theme aligned with luxiedge.com */
:root {
  --bg: #ffffff;
  --bg-subtle: #f9fafb;
  --bg-elev: #f3f4f6;
  --bg-card: #ffffff;
  --border: #e5e7eb;
  --text: #111827;
  --muted: #374151;
  --faint: #6b7280;
  --accent: #059669;
  --accent-dim: #d1fae5;
  --accent-fg: #065f46;
  --warn-bg: #fef3c7;
  --warn-border: #f59e0b;
  --link: #059669;
  --btn-bg: #111827;
  --btn-fg: #ffffff;
  --btn-hover: #374151;
  --radius: 12px;
  --max: 1080px;
  --font: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
  --display: "Space Grotesk", "Inter", system-ui, sans-serif;
  --mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, monospace;
}

* { box-sizing: border-box; margin: 0; padding: 0; }

html { scroll-behavior: smooth; }

body {
  font-family: var(--font);
  background: var(--bg);
  color: var(--muted);
  line-height: 1.65;
  font-size: 17px;
  min-height: 100vh;
  -webkit-font-smoothing: antialiased;
}

a { color: var(--link); text-decoration: none; }
a:hover { text-decoration: underline; }

.wrap { max-width: var(--max); margin: 0 auto; padding: 0 1.5rem; }

/* Nav */
.site-header {
  border-bottom: 1px solid var(--border);
  background: rgba(255, 255, 255, 0.92);
  backdrop-filter: blur(10px);
  position: sticky;
  top: 0;
  z-index: 50;
}
.nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 1rem 0;
  flex-wrap: wrap;
}
.brand {
  font-family: var(--display);
  font-weight: 700;
  letter-spacing: -0.03em;
  color: var(--text);
  font-size: 1.3rem;
  text-decoration: none !important;
}
.brand span { color: var(--accent); }
.nav-links {
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem 1.25rem;
  list-style: none;
  align-items: center;
}
.nav-links a {
  color: var(--faint);
  font-size: 0.85rem;
  text-decoration: none !important;
}
.nav-links a:hover,
.nav-links a[aria-current="page"] { color: var(--text); }
.nav-links .cta-nav {
  background: var(--btn-bg);
  color: var(--btn-fg) !important;
  padding: 0.4rem 0.85rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.85rem;
}
.nav-links .cta-nav:hover { background: var(--btn-hover); }

/* Hero */
.hero {
  padding: 3.25rem 0 2.5rem;
  border-bottom: 1px solid var(--border);
  background: var(--bg-subtle);
}
.eyebrow {
  color: var(--accent);
  font-size: 0.8rem;
  font-weight: 600;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  margin-bottom: 0.75rem;
}
.hero h1 {
  font-family: var(--display);
  font-size: clamp(1.85rem, 4.2vw, 2.6rem);
  line-height: 1.15;
  letter-spacing: -0.03em;
  max-width: 20ch;
  margin-bottom: 1rem;
  color: var(--text);
}
.hero .lede {
  color: var(--muted);
  font-size: 1.1rem;
  max-width: 52ch;
  margin-bottom: 1.75rem;
}
.cta-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  margin-bottom: 2rem;
}
.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.7rem 1.15rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.92rem;
  text-decoration: none !important;
  border: 1px solid transparent;
  cursor: pointer;
}
.btn-primary {
  background: var(--btn-bg);
  color: var(--btn-fg) !important;
}
.btn-primary:hover { background: var(--btn-hover); }
.btn-secondary {
  background: transparent;
  color: var(--text) !important;
  border-color: var(--border);
}
.btn-secondary:hover { border-color: var(--faint); }
.btn-ghost {
  background: var(--accent-dim);
  color: var(--accent-fg) !important;
}

/* Metric strip */
.metrics {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  gap: 0.75rem;
}
.metric {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1rem 1.1rem;
}
.metric .val {
  font-family: var(--display);
  font-size: 1.55rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  color: var(--accent);
  font-variant-numeric: tabular-nums;
}
.metric .label {
  font-size: 0.82rem;
  color: var(--muted);
  margin-top: 0.2rem;
}
.metric .src {
  font-size: 0.75rem;
  color: var(--faint);
  margin-top: 0.45rem;
}
.metric .src a { color: var(--faint); }

/* Sections */
section { padding: 2.75rem 0; border-bottom: 1px solid var(--border); }
section h2 {
  font-family: var(--display);
  font-size: 1.5rem;
  letter-spacing: -0.02em;
  margin-bottom: 0.75rem;
  color: var(--text);
}
section .sub {
  color: var(--faint);
  max-width: 60ch;
  margin-bottom: 1.5rem;
}

.grid-2 {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}
@media (max-width: 720px) {
  .grid-2 { grid-template-columns: 1fr; }
}

.card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1.25rem 1.35rem;
}
.card h3 {
  font-family: var(--display);
  font-size: 1.05rem;
  margin-bottom: 0.45rem;
  color: var(--text);
}
.card p, .card li { color: var(--muted); font-size: 0.95rem; }
.card ul { padding-left: 1.15rem; margin-top: 0.5rem; }
.card li { margin: 0.3rem 0; }
.card.accent {
  border-color: #a7f3d0;
  background: linear-gradient(160deg, var(--accent-dim), #ffffff);
}

/* Tables */
.table-wrap { overflow-x: auto; margin: 1rem 0; }
table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.92rem;
}
th, td {
  text-align: left;
  padding: 0.65rem 0.75rem;
  border-bottom: 1px solid var(--border);
}
th {
  color: var(--faint);
  font-weight: 600;
  font-size: 0.78rem;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  background: var(--bg-subtle);
}
td { font-variant-numeric: tabular-nums; color: var(--muted); }
tr:hover td { background: var(--bg-subtle); }
tr.row-highlight td {
  background: var(--accent-dim);
  color: var(--text);
  font-weight: 600;
}

/* Callouts */
.callout {
  border-left: 3px solid var(--accent);
  background: var(--accent-dim);
  padding: 0.9rem 1.1rem;
  border-radius: 0 var(--radius) var(--radius) 0;
  margin: 1.25rem 0;
  color: var(--muted);
  font-size: 0.95rem;
}
.callout.warn {
  border-left-color: var(--warn-border);
  background: var(--warn-bg);
}
.callout strong { color: var(--text); }

.pill-row { display: flex; flex-wrap: wrap; gap: 0.4rem; margin: 1rem 0; }
.pill {
  font-size: 0.78rem;
  padding: 0.25rem 0.65rem;
  border-radius: 999px;
  border: 1px solid var(--border);
  color: var(--faint);
  background: var(--bg-subtle);
}

/* Footnotes */
.footnote, .method {
  font-size: 0.85rem;
  color: var(--faint);
  margin-top: 1rem;
  max-width: 70ch;
}
.method code, code {
  font-family: var(--mono);
  font-size: 0.88em;
  background: var(--bg-elev);
  padding: 0.1em 0.35em;
  border-radius: 4px;
  color: var(--text);
}
pre {
  background: var(--bg-elev);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1rem 1.1rem;
  overflow-x: auto;
  font-family: var(--mono);
  font-size: 0.85rem;
  color: var(--muted);
  margin: 1rem 0;
}

/* Footer */
.site-footer {
  padding: 2rem 0 3rem;
  color: var(--faint);
  font-size: 0.88rem;
}
.site-footer a { color: var(--muted); }
.footer-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem 2rem;
  justify-content: space-between;
  align-items: flex-start;
}

/* Page title (inner pages) */
.page-hero {
  padding: 2.5rem 0 1.5rem;
  border-bottom: 1px solid var(--border);
  background: var(--bg-subtle);
}
.page-hero h1 {
  font-family: var(--display);
  font-size: clamp(1.65rem, 3.5vw, 2.15rem);
  letter-spacing: -0.02em;
  margin-bottom: 0.6rem;
  color: var(--text);
}
.page-hero .lede { color: var(--muted); max-width: 55ch; }

/* Evidence list */
.pack {
  display: block;
  padding: 1.1rem 1.2rem;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-card);
  margin-bottom: 0.75rem;
  text-decoration: none !important;
  transition: border-color 0.15s, box-shadow 0.15s;
}
.pack:hover {
  border-color: #6ee7b7;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}
.pack h3 { color: var(--text); margin-bottom: 0.25rem; font-family: var(--display); }
.pack p { color: var(--muted); font-size: 0.92rem; margin: 0; }
.pack .tag {
  display: inline-block;
  font-size: 0.72rem;
  color: var(--accent-fg);
  margin-bottom: 0.35rem;
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}

.two-path {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  margin-top: 1.5rem;
}
@media (max-width: 720px) {
  .two-path { grid-template-columns: 1fr; }
}
.two-path .card h3 { color: var(--accent); }

strong { color: var(--text); font-weight: 600; }

/* Three-leg home grid */
.two-path.three-leg,
.two-path[style*="grid-template-columns: 1fr 1fr 1fr"] {
  grid-template-columns: 1fr 1fr 1fr;
}
@media (max-width: 900px) {
  .two-path[style*="grid-template-columns: 1fr 1fr 1fr"] {
    grid-template-columns: 1fr;
  }
  .nav-links { gap: 0.25rem 0.75rem; }
  .nav-links a { font-size: 0.8rem; }
}
__LUXI_assets_site_css__

echo "Copying into static/ for server.py..."
for f in index.html quant-research.html data-centers.html help.html proof.html contact.html product.html energy.html evidence.html benchmarks.html download.html; do
  [ -f "$f" ] && cp -f "$f" static/
done
mkdir -p static/assets
cp -f assets/site.css static/assets/site.css
echo "=== Done ==="
grep -o 'We need your help' index.html help.html | head -5
ls -la static/help.html static/proof.html static/assets/site.css
echo "Stop + Run the Repl (server.py), then Deploy."
