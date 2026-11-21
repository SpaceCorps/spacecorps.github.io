// Generates the static HTML pages (index, about, contact, privacy, 404, blog/*) from data/ and content/.
// Run: pnpm run gen   (the build runs it first). The generated .html files are committed so Vite+ can use
// them as entry points and reviewers can read them. Edit this file, never the generated pages.
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";

const stats = JSON.parse(readFileSync("data/stats.json", "utf8"));
const clis = JSON.parse(readFileSync("data/clis.json", "utf8"));
const SITE = "https://spacecorps.tech";
const GH = "https://github.com/SpaceCorps";
const esc = (s) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
const fmtDate = (iso) => new Date(iso).toLocaleDateString("en-GB", { day: "numeric", month: "short", year: "numeric", timeZone: "UTC" });
const weeks = Math.max(1, Math.round((new Date(stats.game.latestDate) - new Date(stats.game.first)) / (7 * 864e5)));
const arrow = `<svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 12 12 4M5.5 4H12v6.5"/></svg>`;

const MARK = `<svg viewBox="0 0 64 64" aria-hidden="true"><defs><mask id="crescent"><rect width="64" height="64" fill="#fff"/><circle cx="39" cy="27" r="19" fill="#000"/></mask></defs><circle cx="29" cy="35" r="23" fill="#00cc92" mask="url(#crescent)"/><path d="M5 47C21 60 52 53 58 23" fill="none" stroke="#00cc92" stroke-width="3.4" stroke-linecap="round"/><circle cx="57" cy="17" r="5.6" fill="currentColor"/></svg>`;

const NAV = [
  ["Products", "/#products"],
  ["Open source", "/#open-source"],
  ["Design system", "/#design-system"],
  ["Blog", "/blog/"],
  ["About", "/about.html"],
  ["Contact", "/contact.html"],
];

function shell({ title, desc, path, body, cur, og = "/og-image.png", jsonld = "", robots = "", bodyClass = "" }) {
  const links = (cls) => NAV.map(([t, h]) => `<a href="${h}"${cur === h ? ' aria-current="page"' : ""}>${t}</a>`).join("");
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>${esc(title)}</title>
    <meta name="description" content="${esc(desc)}" />${robots}
    <link rel="canonical" href="${SITE}${path}" />
    <meta name="theme-color" content="#04070a" />
    <meta property="og:site_name" content="SpaceCorps" />
    <meta property="og:title" content="${esc(title)}" />
    <meta property="og:description" content="${esc(desc)}" />
    <meta property="og:type" content="website" />
    <meta property="og:url" content="${SITE}${path}" />
    <meta property="og:image" content="${SITE}${og}" />
    <meta property="og:image:width" content="1200" />
    <meta property="og:image:height" content="630" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:image" content="${SITE}${og}" />
    <link rel="icon" href="/favicon.ico" sizes="32x32" />
    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
    <link rel="apple-touch-icon" href="/apple-touch-icon.png" />
    <link rel="alternate" type="text/markdown" href="/index.md" />
    <link rel="preload" href="/fonts/geist-latin.woff2" as="font" type="font/woff2" crossorigin />
    <script>try{var t=localStorage.getItem("sc-theme");if(t?t==="dark":matchMedia("(prefers-color-scheme: dark)").matches)document.documentElement.classList.add("dark")}catch(e){}</script>
    <script type="module" src="/src/main.js"></script>${jsonld}
  </head>
  <body${bodyClass ? ` class="${bodyClass}"` : ""}>
    <a class="skip" href="#main">Skip to content</a>
    <header class="nav"><div class="wrap">
      <a class="brand" href="/" aria-label="SpaceCorps home">${MARK}<span>Space<b>Corps</b></span></a>
      <nav class="nav-links" aria-label="Main">${links()}</nav>
      <a class="btn sm cta-sm" href="https://play.spacecorps.tech/">Play free</a>
      <button class="icon-btn theme-btn" type="button" aria-label="Toggle dark mode"><svg class="sun" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" aria-hidden="true"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg><svg class="moon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z"/></svg></button>
      <details class="menu"><summary class="icon-btn" aria-label="Menu"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" aria-hidden="true"><path d="M4 7h16M4 12h16M4 17h16"/></svg></summary><nav class="panel" aria-label="Main (mobile)">${links()}<a href="https://play.spacecorps.tech/">Play SpaceCorps 2027</a></nav></details>
    </div></header>
    <main id="main">
${body}
    </main>
    <footer class="site"><div class="wrap">
      <div class="foot">
        <div class="about"><a class="brand" href="/" aria-label="SpaceCorps home">${MARK}<span>Space<b>Corps</b></span></a><p>SpaceCorps Technology AB. A small Swedish studio building games, a game engine, business software and developer tools in Rust.</p></div>
        <div><p class="fh">Products</p><ul><li><a href="https://play.spacecorps.tech/">SpaceCorps 2027</a></li><li><a href="https://space3d.dev/">Space3d</a></li><li><a href="https://obop.dev/">obop</a></li><li><a href="${GH}/swindle-releases/releases">Swindle</a></li></ul></div>
        <div><p class="fh">Build</p><ul><li><a href="/#open-source">Rust CLI tools</a></li><li><a href="https://storybook.spacecorps.tech">Design system</a></li><li><a href="${GH}">GitHub</a></li><li><a href="/blog/">Blog</a></li></ul></div>
        <div><p class="fh">Company</p><ul><li><a href="/about.html">About</a></li><li><a href="/contact.html">Contact</a></li><li><a href="/status.html">Status</a></li><li><a href="/privacy.html">Privacy</a></li></ul></div>
        <div><p class="fh">Reach us</p><ul><li><a href="mailto:hello@spacecorpstechnology.com">hello@spacecorpstechnology.com</a></li><li><a href="mailto:security@spacecorpstechnology.com">security@spacecorpstechnology.com</a></li><li><a href="https://discord.gg/VjW67tkrTb">Discord</a></li></ul></div>
      </div>
      <div class="legal"><span>&copy; 2026 SpaceCorps Technology AB, Sweden</span><span>No cookies. No trackers. No third-party requests.</span></div>
    </div></footer>
  </body>
</html>
`;
}

const out = (file, html) => { mkdirSync(file.includes("/") ? file.slice(0, file.lastIndexOf("/")) : ".", { recursive: true }); writeFileSync(file, html, { encoding: "utf8" }); };

/* ---------------- Home ---------------- */
const picture = (name, w, alt, sizes, extra = "") =>
  `<img src="/img/${name}-${w[0]}.webp" srcset="${w.map((x) => `/img/${name}-${x}.webp ${x}w`).join(", ")}" sizes="${sizes}" alt="${alt}" ${extra} />`;

const catOrder = [...new Set(clis.map((c) => c.cat))];
const cliItems = clis.map((c) => {
  const s = stats.cli[c.repo] ?? {};
  const meta = `<span>${s.license ?? "MIT"}</span><span>Rust</span>${s.release ? `<span class="rel">${s.release}</span>` : ""}`;
  return `<li data-cat="${esc(c.cat)}" data-s="${esc((c.name + " " + c.desc + " " + c.cat + " " + c.repo).toLowerCase())}"><a href="${GH}/${c.repo}"><span class="nm">${esc(c.name)}</span><span class="ds">${esc(c.desc)}</span><span class="mt">${meta}</span></a></li>`;
}).join("\n          ");
const filterBtns = [["all", "All 24"], ...catOrder.map((c) => [c, `${c} ${clis.filter((x) => x.cat === c).length}`])]
  .map(([v, t], i) => `<button type="button" data-cat="${esc(v)}" aria-pressed="${i === 0}">${esc(t)}</button>`).join("");

const SECTION = (n, id, label, h, sub) => `<div class="sec-head"><p class="eyebrow"><span class="dot"></span>${n} / ${label}</p><h2 id="${id}-h">${h}</h2>${sub ? `<p class="sub">${sub}</p>` : ""}</div>`;

const posts = [
  { slug: "spacecorps-2027", tag: "Game", d: "Three companies, 16 sectors, clans, crafting and 30-day seasons. Free on macOS, Windows and Linux.", t: "SpaceCorps 2027 is out: a free space MMO built on our own engine" },
  { slug: "space3d-engine", tag: "Engine", d: "A million instances at ~1,780 fps, async that never blocks a frame, and a headless harness agents can drive.", t: "Space3d: a Rust 3D engine built for AI agents" },
  { slug: "obop", tag: "Product", d: "An open-source business platform built for people and AI agents.", t: "obop: one business, one platform" },
  { slug: "getelec-spaceemit", tag: "Case study", d: "Custom Rust physics software for a client: 17 µs per point instead of 650 ms, with matching results.", t: "47,000× faster electron emission: spaceemit vs GETELEC" },
];

const home = `
      <section class="hero space" aria-labelledby="hero-h">
        <div class="hero-panel">
          <div class="hero-bg"></div><div class="hero-grid"></div><canvas id="stars" aria-hidden="true"></canvas>
          <div class="hero-ship" aria-hidden="true"><img src="/img/hero-ship-1190.webp" srcset="/img/hero-ship-640.webp 640w, /img/hero-ship-1190.webp 1190w" sizes="(max-width:900px) 100vw, 62vw" width="1190" height="800" alt="" fetchpriority="high" decoding="async" /></div>
          <div class="hero-copy">
            <p class="eyebrow"><span class="dot"></span>SpaceCorps Technology AB &middot; Sweden</p>
            <h1 id="hero-h">Space games, engines and tools. <span class="grad">Built in Rust.</span></h1>
            <p class="lead">We make a free space MMO, the engine it runs on, an open-source business platform and a fleet of command-line tools. A small team, plus AI agents, shipping in the open.</p>
            <div class="cta"><a class="btn" href="https://play.spacecorps.tech/">Play SpaceCorps 2027 ${arrow}</a><a class="btn o" href="#products">Explore the products</a></div>
          </div>
          <dl class="telemetry">
            <div><dt>Latest build</dt><dd>${stats.game.latest}<small>${fmtDate(stats.game.latestDate)}</small></dd></div>
            <div><dt>Game releases</dt><dd>${stats.game.releases}<small>in ${weeks} weeks</small></dd></div>
            <div><dt>Platforms</dt><dd>macOS &middot; Win &middot; Linux<small>Android, web</small></dd></div>
            <div><dt>Engine</dt><dd>Space3d<small>Rust &middot; Apache-2.0</small></dd></div>
          </dl>
        </div>
      </section>

      <section class="sec" id="products" aria-labelledby="products-h"><div class="wrap">
        ${SECTION("01", "products", "Products", "One studio. Four things you can use today.", "Each card says plainly where the product stands.")}
        <div class="bento">
          <article class="card hud span-7 flagship">
            <div class="shot">${picture("combat", [800, 1600], "A SpaceCorps 2027 fleet battle: ships trading fire against aliens in deep space", "(max-width:980px) 100vw, 700px", 'width="1600" height="900" loading="lazy" decoding="async"')}</div>
            <div class="card-body">
              <div class="card-top"><span class="tag live"><span class="dot"></span>Live</span><span class="tag">Game</span><span class="meta">v${stats.game.latest.replace(/^v/, "")} &middot; ${fmtDate(stats.game.latestDate)}</span></div>
              <h3><a href="https://play.spacecorps.tech/">SpaceCorps 2027</a></h3>
              <p>A free-to-play space MMO: fly for a corporation, hunt aliens with your clan and climb the season ranking. Desktop clients for macOS, Windows and Linux, an Android build and a browser build, all on our own engine.</p>
              <div class="links"><a class="primary" href="https://play.spacecorps.tech/">Download</a><a href="https://web.spacecorps.xyz/">Play in browser</a><a href="https://play.spacecorps.tech/patchnotes.html">Patch notes</a><a href="/blog/spacecorps-2027.html">Read the launch post</a></div>
            </div>
          </article>
          <article class="card hud span-5">
            <div class="card-body">
              <div class="card-top"><span class="tag">Engine</span><span class="meta">Rust &middot; Apache-2.0</span></div>
              <h3><a href="https://space3d.dev/">Space3d</a></h3>
              <p>A cross-platform game engine in Rust for native, web and mobile. Deterministic simulation, headless rendering and golden-screenshot tests make it a loop an AI agent can verify. It runs SpaceCorps 2027.</p>
              <pre class="term"><span class="c"># no binary releases yet: build from source</span>
<span class="p">$</span> cargo install --git ${GH}/Space3d space3d-cli
<span class="p">$</span> space3d new my_game
<span class="p">$</span> space3d run</pre>
              <div class="links"><a href="https://space3d.dev/">space3d.dev</a><a href="${GH}/Space3d">Source</a><a href="/blog/space3d-engine.html">How it works</a></div>
            </div>
            <div class="shot wide">${picture("station", [800, 1600], "A space station in SpaceCorps 2027, rendered by Space3d", "(max-width:980px) 100vw, 500px", 'width="1600" height="900" loading="lazy" decoding="async"')}</div>
          </article>
          <article class="card hud span-4">
            <div class="card-body">
              <div class="card-top"><span class="tag">Business software</span><span class="tag">Pre-1.0</span></div>
              <h3><a href="https://obop.dev/">obop</a></h3>
              <p>One business, one platform: an open-source business OS in Rust and Vite+, with sales, CRM, invoicing and stock as modules on one data model. Agents propose, humans approve. Early code, not a finished product.</p>
              <div class="chips" role="list" aria-label="obop modules"><span class="chip on" role="listitem">CRM</span><span class="chip on" role="listitem">Sales</span><span class="chip on" role="listitem">Invoicing</span><span class="chip on" role="listitem">Contacts</span><span class="chip on" role="listitem">Products</span><span class="chip off" role="listitem">Accounting</span><span class="chip off" role="listitem">Inventory</span><span class="chip off" role="listitem">+ marketplace</span></div>
              <p class="meta">Solid: available. Dashed: marketplace modules.</p>
              <div class="links"><a href="https://obop.dev/">obop.dev</a><a href="${GH}/obop">Source</a></div>
            </div>
          </article>
          <article class="card hud span-4">
            <div class="shot">${picture("molecular", [640, 1200], "Illustration of electron field emission from a nanotip between anode and cathode", "(max-width:980px) 100vw, 400px", 'width="1080" height="1080" loading="lazy" decoding="async"').replace("<img", '<img style="object-position:50% 38%"')}</div>
            <div class="card-body">
              <div class="card-top"><span class="tag">Simulation</span><span class="tag">Client work</span></div>
              <h3><a href="/blog/getelec-spaceemit.html">Space3d-Molecular</a></h3>
              <p>Molecular dynamics (<code>spacemd</code>) and thermal-field electron emission (<code>spaceemit</code>) on Space3d.</p>
              <div class="bigstat">47,000&times;<small>faster than GETELEC, 2,456 points, all cores</small></div>
              <div class="links"><a href="/blog/getelec-spaceemit.html">Case study</a><a href="${GH}/Space3d-Molecular">API docs</a></div>
            </div>
          </article>
          <article class="card hud span-4">
            <div class="card-body">
              <div class="card-top"><span class="tag">Game</span><span class="tag">macOS</span></div>
              <h3><a href="${GH}/swindle-releases/releases">Swindle</a></h3>
              <p>A trading game with on-device AI NPCs: 100 days to hit the quota in a bazaar of LLM-driven characters. Runs on a Mac; iPhone and iPad are planned.</p>
              <div class="chips"><span class="chip">On-device AI</span><span class="chip">Release 0.1.0</span><span class="chip off">iPhone &amp; iPad planned</span></div>
              <div class="links"><a href="${GH}/swindle-releases/releases">Releases</a><a href="${GH}/open-apple-models">open-apple-models</a></div>
            </div>
          </article>
        </div>
      </div></section>

      <section class="sec" aria-labelledby="proof-h"><div class="wrap">
        ${SECTION("02", "proof", "Track record", "We ship, and the history is public.", "Counts come straight from the GitHub API, not estimates.")}
        <div class="proof">
          <div><div class="n">${stats.org.publicRepos}</div><div class="l">active public repositories under github.com/SpaceCorps</div></div>
          <div><div class="n">${stats.org.rustRepos}</div><div class="l">of them written in Rust</div></div>
          <div><div class="n">${stats.org.openLicensed}</div><div class="l">under the MIT or Apache-2.0 licence</div></div>
          <div><div class="n">${stats.game.releases}</div><div class="l">game releases in ${weeks} weeks, latest ${stats.game.latest}</div></div>
        </div>
        <p class="proof-note">Source: GitHub API, fetched ${fmtDate(stats.generatedAt)} and committed in data/stats.json. Archived repositories and forks are not counted.</p>
      </div></section>

      <section class="sec" id="open-source" aria-labelledby="oss-h"><div class="wrap">
        ${SECTION("03", "oss", "Open source", "24 command-line tools, all Rust, all MIT.", "Built for people and for agents: readable output by default, JSON with <code>--json</code>. Credentials live in your OS keystore and no tool sends telemetry. Each links to its repository with install notes and releases.")}
        <div class="dir-tools">
          <div class="search"><svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"><circle cx="7" cy="7" r="4.5"/><path d="m10.5 10.5 3 3"/></svg><label class="vh" for="q">Search the command-line tools</label><input id="q" type="search" placeholder="Search tools: dns, scrape, mail&hellip;" autocomplete="off" spellcheck="false" /><kbd aria-hidden="true">/</kbd></div>
          <div class="filters" role="group" aria-label="Filter by category">${filterBtns}</div>
        </div>
        <p class="count" id="count" role="status" aria-live="polite">24 command-line tools</p>
        <ul class="dir" id="dir">
          ${cliItems}
        </ul>
        <p class="empty" id="empty" hidden>No tool matches that search. Try a different word, or <a href="${GH}">browse every repository on GitHub</a>.</p>
        <div class="also">
          <h3>Libraries and apps</h3>
          <ul>
            <li><a href="${GH}/DiskLens">DiskLens</a>, a desktop disk-space explorer</li>
            <li><a href="${GH}/open-apple-models">open-apple-models</a>, on-device game AI for Apple</li>
            <li><a href="${GH}/open-android-models">open-android-models</a>, the same for Android</li>
            <li><a href="${GH}/open-accounting-sweden">open-accounting-sweden</a>, agentic bookkeeping</li>
            <li><a href="${GH}/open-browser">open-browser</a>, agentic browser automation</li>
            <li><a href="${GH}/open-agents">open-agents</a>, a CLI-first agent runner</li>
            <li><a href="${GH}/open-glass">open-glass</a>, Apple Glass UI for the web</li>
            <li><a href="${GH}">All repositories on GitHub</a></li>
          </ul>
        </div>
      </div></section>

      <section class="sec" id="design-system" aria-labelledby="ds-h"><div class="wrap">
        <div class="ds-grid">
          <div>
            ${SECTION("04", "ds", "Design system", "One system, every screen.", `Our sites and apps share <code>@spacecorps/components</code>: React components, renderers and widgets. The colours and type on this page come straight from its tokens: brand green, zinc neutrals, Geist. Browse every component in Storybook.`)}
            <div class="cta-row"><a class="btn" href="https://storybook.spacecorps.tech">Open Storybook ${arrow}</a><a class="btn o" href="${GH}/storybook">Source and install notes</a></div>
            ${stats.designSystem.latest ? `<p class="proof-note">Latest release ${stats.designSystem.latest}, MIT.</p>` : ""}
          </div>
          <div class="ds-stage" aria-label="Design token preview">
            <div class="swatches" role="img" aria-label="Colour tokens: brand green, foreground, muted foreground, muted, card, background">
              <span style="background:var(--sc-green);color:#000">sc-green</span><span style="background:var(--foreground);color:var(--background)">foreground</span><span style="background:var(--muted-foreground);color:var(--background)">muted-fg</span><span style="background:var(--muted)">muted</span><span style="background:var(--card);border-left:1px solid var(--line)">card</span><span style="background:var(--background);border-left:1px solid var(--line)">bg</span>
            </div>
            <div class="ds-demo"><a class="btn sm" href="https://storybook.spacecorps.tech">Button</a><a class="btn o sm" href="https://storybook.spacecorps.tech">Outline</a><span class="badge">Badge</span><span class="field">Input field</span></div>
            <div class="type-spec"><span class="aa">Aa</span><span>Geist Variable &middot; Geist Mono Variable<br />self-hosted, latin subset</span></div>
          </div>
        </div>
      </div></section>

      <section class="sec defer" aria-labelledby="how-h"><div class="wrap">
        ${SECTION("05", "how", "How we build", "Small team. Strong defaults.")}
        <div class="principles">
          <div><div class="k">01</div><h3>Rust first</h3><p>Services, the engine and the tools are Rust. The web is TypeScript on Vite+.</p></div>
          <div><div class="k">02</div><h3>Agent-friendly</h3><p>Every CLI speaks plain text for people and <code>--json</code> for agents. Engines come with headless test loops.</p></div>
          <div><div class="k">03</div><h3>Everything in a container</h3><p>If it runs, it ships as a Docker image. This site is one: Caddy serving static files.</p></div>
          <div><div class="k">04</div><h3>Open by default</h3><p>Source, releases and decisions are public. No cookies or trackers on our sites.</p></div>
        </div>
      </div></section>

      <section class="sec defer" aria-labelledby="blog-h"><div class="wrap">
        ${SECTION("06", "blog", "Blog", "Notes from the build.")}
        <div class="posts">${posts.map((p) => `<article class="card hud post-card"><div class="card-body"><div class="card-top"><span class="tag">${p.tag}</span><span class="meta">16 Oct 2026</span></div><h3><a href="/blog/${p.slug}.html">${esc(p.t)}</a></h3><p>${esc(p.d)}</p></div></article>`).join("")}</div>
      </div></section>

      <section class="band space wrap" aria-labelledby="contact-h">
        <div class="hero-panel">
          <div class="hero-bg"></div><div class="hero-grid"></div>
          <div><p class="eyebrow"><span class="dot"></span>Contact</p><h2 id="contact-h">Questions, bugs, partnerships?</h2><p>Bugs and feature requests are fastest in the product's issue tracker. For everything else, write to us.</p></div>
          <div class="addr"><a href="mailto:hello@spacecorpstechnology.com">hello@spacecorpstechnology.com<small>General</small></a><a href="mailto:security@spacecorpstechnology.com">security@spacecorpstechnology.com<small>Security</small></a><a href="/contact.html">All the ways to reach us<small>&rarr;</small></a></div>
        </div>
      </section>`;

const jsonld = `
    <script type="application/ld+json">${JSON.stringify({ "@context": "https://schema.org", "@type": "Organization", name: "SpaceCorps Technology AB", url: SITE, logo: `${SITE}/apple-touch-icon.png`, sameAs: [GH], description: "Swedish software company building the SpaceCorps 2027 space MMO, the Space3d engine, obop and open-source Rust tools." })}</script>`;

out("index.html", shell({
  title: "SpaceCorps | Games, engines and software in Rust",
  desc: "SpaceCorps Technology AB builds the SpaceCorps 2027 space MMO, the Space3d engine, the obop open-source business platform and 24 open-source Rust command-line tools.",
  path: "/", body: home, jsonld,
}));

/* ---------------- Inner pages ---------------- */
const head = (h, lead) => `      <div class="wrap"><header class="page-head"><p class="eyebrow"><span class="dot"></span>SpaceCorps</p><h1>${h}</h1>${lead ? `<p class="lead">${lead}</p>` : ""}</header>`;
const miniCard = (tag, name, text, href) => `<article class="card hud"><div class="card-body"><span class="tag">${tag}</span><h3><a href="${href}">${name}</a></h3><p>${text}</p></div></article>`;

out("about.html", shell({
  title: "About | SpaceCorps", cur: "/about.html", path: "/about.html",
  desc: "SpaceCorps Technology AB: a small Swedish company building games, the Space3d engine, business software and open-source Rust tools.",
  body: `${head("A small studio with its own engine.", "SpaceCorps Technology AB is a small Swedish software company. We build games, a game engine, business software and developer tools, mostly in Rust.")}
      <div class="two">
        ${miniCard("Game", "SpaceCorps 2027", "A free-to-play space MMO for macOS, Windows, Linux, Android and the browser.", "https://play.spacecorps.tech/")}
        ${miniCard("Engine", "Space3d", "The Rust game engine the game runs on, also used for molecular simulation.", "https://space3d.dev/")}
        ${miniCard("Business software", "obop", "An open-source business platform in early development.", "https://obop.dev/")}
        ${miniCard("Open source", "24 Rust CLI tools", "Plus several libraries, all on GitHub.", "/#open-source")}
      </div>
      <div class="prose">
        <h2>How we work</h2>
        <p>A small team plus AI agents. Rust for services and performance-critical code, TypeScript on Vite+ for the web, everything shipped as containers. Every product site uses the same <a href="https://storybook.spacecorps.tech">design system</a>.</p>
        <h2>Custom physics software</h2>
        <p>We also build software for researchers. See the <a href="/blog/getelec-spaceemit.html">spaceemit case study</a>: electron-emission modelling 47,000&times; faster than the reference tool.</p>
        <h2>Elsewhere</h2>
        <p><a href="https://play.spacecorps.tech/">play.spacecorps.tech</a> game downloads &middot; <a href="https://web.spacecorps.xyz/">web.spacecorps.xyz</a> browser game &middot; <a href="https://space3d.dev/">space3d.dev</a> &middot; <a href="https://obop.dev/">obop.dev</a> &middot; <a href="https://storybook.spacecorps.tech">storybook.spacecorps.tech</a> &middot; <a href="${GH}">GitHub</a></p>
      </div></div>`,
}));

out("contact.html", shell({
  title: "Contact | SpaceCorps", cur: "/contact.html", path: "/contact.html",
  desc: "How to reach SpaceCorps: email, issue trackers, Discord and private vulnerability reporting.",
  body: `${head("Get in touch.", "The fastest way to reach us is where the work happens. For anything else, email.")}
      <div class="two">
        <article class="card hud"><div class="card-body"><span class="tag">General</span><h3><a href="mailto:hello@spacecorpstechnology.com">hello@spacecorpstechnology.com</a></h3><p>Partnerships, press, custom software, anything that is not a bug report.</p></div></article>
        <article class="card hud"><div class="card-body"><span class="tag">Security</span><h3><a href="mailto:security@spacecorpstechnology.com">security@spacecorpstechnology.com</a></h3><p>Vulnerability reports. You can also use GitHub's private &ldquo;Report a vulnerability&rdquo; on the affected repository.</p></div></article>
      </div>
      <div class="prose">
        <h2>Questions, bugs and feature requests</h2>
        <ul>
          <li><strong>Game:</strong> <a href="${GH}/play/issues">SpaceCorps 2027 issues</a> or our <a href="https://discord.gg/VjW67tkrTb">Discord</a>.</li>
          <li><strong>Engine:</strong> <a href="${GH}/Space3d/issues">Space3d issues</a>.</li>
          <li><strong>obop:</strong> <a href="${GH}/obop/issues">obop issues</a>.</li>
          <li><strong>Any other tool:</strong> open an issue in its repository on <a href="${GH}">github.com/SpaceCorps</a>.</li>
        </ul>
        <h2>Security</h2>
        <p>Please report vulnerabilities privately, by email or with GitHub's &ldquo;Report a vulnerability&rdquo; (Security tab), not in a public issue.</p>
        <h2>Company</h2>
        <p>SpaceCorps Technology AB, Sweden.</p>
      </div></div>`,
}));

out("status.html", shell({
  title: "Status | SpaceCorps", cur: "/status.html", path: "/status.html",
  desc: "How many pilots are online on the SpaceCorps 2027 game servers: now, today, this week and month, the last 3 months, the last year and since the very beginning.",
  body: `${head("Players online.", "Pilots connected to the SpaceCorps 2027 game servers, recorded every minute. <a href=\"https://play.spacecorps.tech/\">Play free</a> and add yourself.")}
      <section class="status" id="status" data-endpoint="/api/status/series" aria-label="Players online over time">
        <div class="st-now" id="st-now" aria-live="polite"></div>
        <div class="st-range" role="group" aria-label="Time range">
          <button type="button" data-range="day" aria-pressed="true">Today</button>
          <button type="button" data-range="week" aria-pressed="false">Week</button>
          <button type="button" data-range="month" aria-pressed="false">Month</button>
          <button type="button" data-range="quarter" aria-pressed="false">3 months</button>
          <button type="button" data-range="year" aria-pressed="false">Year</button>
          <button type="button" data-range="all" aria-pressed="false">All time</button>
        </div>
        <figure class="st-card">
          <figcaption id="st-caption"></figcaption>
          <div class="st-plot" id="st-plot" tabindex="0" role="group" aria-label="Line chart of players online per server. Use the left and right arrow keys to read values, or open the table below."><p class="st-msg">Loading&hellip;</p></div>
          <p class="st-note" id="st-note"></p>
          <p class="st-note"><span class="st-legend-down"></span>A red band marks a time when a server was down or updating.</p>
        </figure>
        <details class="st-table"><summary>Show as table</summary><div id="st-table"></div></details>
        <noscript><p class="st-note">The chart needs JavaScript. The numbers behind it are at <a href="/api/status/series?range=day">/api/status/series</a>.</p></noscript>
      </section></div>`,
}));

out("privacy.html", shell({
  title: "Privacy | SpaceCorps", path: "/privacy.html",
  desc: "spacecorps.tech sets no cookies, runs no analytics and loads nothing from third parties.",
  body: `${head("This site collects nothing about you.")}
      <div class="prose">
        <h2>This website</h2>
        <p>spacecorps.tech sets no cookies, runs no analytics or trackers and loads no third-party scripts, fonts or images. The only thing we store is your light or dark theme choice, in your own browser, if you use the theme switch. Our web server keeps standard request logs (address, time, page) for security and operations, which we do not share.</p>
        <h2>The game</h2>
        <p>SpaceCorps 2027 has its own privacy policy, which covers the client, the game server and the download site: <a href="https://play.spacecorps.tech/privacy/">play.spacecorps.tech/privacy</a>. It is operated by SpaceCorps Technology AB.</p>
        <h2>Our command-line tools</h2>
        <p>They send no telemetry. API credentials you give a tool are stored in your operating system's keystore (macOS Keychain, Linux Secret Service or Windows DPAPI) and are sent only to the service the tool talks to. Each tool's repository documents what it contacts.</p>
        <h2>Contact</h2>
        <p>Questions about privacy: <a href="mailto:hello@spacecorpstechnology.com">hello@spacecorpstechnology.com</a> or see <a href="/contact.html">Contact</a>.</p>
      </div></div>`,
}));

out("404.html", shell({
  title: "Page not found | SpaceCorps", path: "/404.html", robots: '\n    <meta name="robots" content="noindex" />',
  desc: "This page does not exist. Head back to the SpaceCorps home page.",
  body: `      <div class="wrap notfound"><div class="code" aria-hidden="true">4<span>0</span>4</div><h1 style="font-size:clamp(1.6rem,3vw,2.4rem);letter-spacing:-.03em;margin:8px 0 12px">Lost in space.</h1><p class="lead" style="color:var(--muted-foreground);max-width:32em">That page is not here. It may have moved, or the link may be wrong.</p><div style="display:flex;flex-wrap:wrap;gap:12px;margin-top:28px"><a class="btn" href="/">Back to home</a><a class="btn o" href="/blog/">Read the blog</a><a class="btn o" href="/#open-source">Browse the tools</a></div></div>`,
}));

/* ---------------- Blog ---------------- */
out("blog/index.html", shell({
  title: "Blog | SpaceCorps", cur: "/blog/", path: "/blog/",
  desc: "Notes from the build: the SpaceCorps 2027 game, the Space3d engine, obop and custom physics software.",
  body: `${head("Notes from the build.", "Games, engines and the Rust that holds them together.")}
      <ul class="post-list" style="margin-top:36px">${posts.map((p) => `<li><article class="card hud"><div class="card-body"><div class="card-top"><span class="tag">${p.tag}</span><span class="meta">16 Oct 2026</span></div><h2><a href="/blog/${p.slug}.html">${esc(p.t)}</a></h2><p>${esc(p.d)}</p></div></article></li>`).join("")}</ul></div>`,
}));

for (const p of posts) {
  const src = readFileSync(`content/blog/${p.slug}.html`, "utf8");
  const g = (k) => src.match(new RegExp(`<!--${k}:(.*?)-->`, "s"))[1];
  const body = src.replace(/<!--[a-z]+:.*?-->\n?/gs, "");
  const title = g("title"), meta = g("meta"), desc = g("desc");
  out(`blog/${p.slug}.html`, shell({
    title: `${title.replace(/&amp;/g, "&")} | SpaceCorps`, desc, path: `/blog/${p.slug}.html`, cur: "/blog/",
    body: `      <article class="post"><a href="/blog/" class="back">&larr; SpaceCorps Blog</a><h1>${title}</h1><p class="meta">${meta}</p>
${body}
      </article>`,
  }));
}
console.log("pages generated");
