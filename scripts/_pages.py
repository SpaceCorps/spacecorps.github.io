"""Generates index/about/contact/privacy/404.html. Run: python scripts/_pages.py"""
import sys

sys.path.insert(0, "scripts")
from _gen import page


def card(tag, name, desc, links):
    ls = "".join(f'<a href="{h}">{t}</a>' for t, h in links)
    return f'<article class="card"><span class="tag">{tag}</span><h3>{name}</h3><p>{desc}</p><div class="links">{ls}</div></article>'


products = "".join(
    [
        card("Game", "SpaceCorps 2027", "A free-to-play space MMO: fly for a corporation, hunt aliens with your clan and climb the season ranking. Desktop clients for macOS, Windows and Linux, an Android build and a browser build, all on our own engine. Version 0.5.9.", [("Download", "https://play.spacecorps.tech/"), ("Play in browser", "https://web.spacecorps.xyz/"), ("Patch notes", "https://play.spacecorps.tech/patchnotes.html")]),
        card("Engine", "Space3d", "A cross-platform game engine in Rust for native, web and mobile, with a Claude Code integration for agent-driven development. Apache-2.0.", [("space3d.dev", "https://space3d.dev/"), ("Source", "https://github.com/SpaceCorps/Space3d")]),
        card("Business software", "obop", "One business, one platform: an open-source business OS in Rust and Vite+, with accounting, invoicing, sales, CRM and stock as modules on one data model. Apache-2.0. Early code, not a finished product.", [("obop.dev", "https://obop.dev/"), ("Source", "https://github.com/SpaceCorps/obop")]),
        card("Simulation", "Space3d-Molecular", "Molecular dynamics (<code>spacemd</code>) and thermal-field electron emission (<code>spaceemit</code>) on Space3d. Public API docs, input formats and examples.", [("Docs", "https://github.com/SpaceCorps/Space3d-Molecular"), ("Case study", "/blog/getelec-spaceemit.html")]),
        card("Game", "Swindle", "Trading game for Mac with on-device AI NPCs: 100 days to hit the quota in a bazaar of LLM-driven characters. iPhone and iPad are planned. Release 0.1.0.", [("Releases", "https://github.com/SpaceCorps/swindle-releases/releases")]),
    ]
)

# (label, repository)
CLIS = [
    ("Sliplane", "Sliplane-Cli"), ("Cloudflare", "Cloudflare-Cli"), ("Spaceship", "Spaceship-Cli"),
    ("Loopia", "Loopia-Cli"), ("Namecheap", "namecheap-cli"), ("Storage", "Storage-Cli"),
    ("Exa", "Exa-Cli"), ("Firecrawl", "Firecrawl-Cli"), ("Tic", "Tic-Cli"), ("Gmail", "Gmail-Cli"),
    ("Notify", "Notify-Cli"), ("Bugsink", "Bugsink-Cli"), ("HeyReach", "Heyreach-Cli"),
    ("Reddit", "Reddit-Cli"), ("Twitter / X", "Twitter-Cli"), ("LinkedIn", "Linkedin-Cli"),
    ("YouTube", "Youtube-Cli"), ("TikTok", "TikTok-Cli"), ("Product Hunt", "Producthunt-Cli"),
    ("Stack Overflow", "Stackoverflow-Cli"), ("Samsung Art Mode", "Samsung-Artmode-Cli"),
    ("GitHub Issue Importer", "Github-Issue-Importer"), ("Claude Sessions", "Claude-Sessions-Cli"),
    ("open-apply", "open-apply"),
]
assert len(CLIS) == 24
clis = "".join(f'<li><a href="https://github.com/SpaceCorps/{r}">{n}</a></li>' for n, r in CLIS)

index = f"""      <section class="hero">
        <h1>Games, engines and software, <em>built in Rust</em>.</h1>
        <p class="lead">SpaceCorps Technology AB builds a space MMO, the Space3d engine it runs on, an open-source business platform and a fleet of command-line tools. Most of it is open source.</p>
        <div class="cta"><a class="btn" href="https://play.spacecorps.tech/">Play SpaceCorps 2027</a><a class="btn o" href="#products">All products</a></div>
      </section>
      <section id="products">
        <h2>Products</h2>
        <p class="sub">What we ship today. Status is stated plainly on each card.</p>
        <div class="grid">{products}</div>
      </section>
      <section id="open-source">
        <h2>Open source</h2>
        <p class="sub">24 command-line tools, all in Rust and MIT-licensed, built for people and for agents: readable output by default, JSON with <code>--json</code>. Each links to its repository, with install notes and releases.</p>
        <ul class="cols">{clis}</ul>
        <p class="sub" style="margin-top:20px">Also: <a href="https://github.com/SpaceCorps/DiskLens">DiskLens</a> (desktop disk-space explorer), <a href="https://github.com/SpaceCorps/open-apple-models">open-apple-models</a> and <a href="https://github.com/SpaceCorps/open-android-models">open-android-models</a> (on-device game AI for Apple and Android), <a href="https://github.com/SpaceCorps/open-accounting-sweden">open-accounting-sweden</a>, <a href="https://github.com/SpaceCorps/open-browser">open-browser</a>, <a href="https://github.com/SpaceCorps/open-agents">open-agents</a> and <a href="https://github.com/SpaceCorps/open-glass">open-glass</a>. Everything is on <a href="https://github.com/SpaceCorps">github.com/SpaceCorps</a>.</p>
      </section>
      <section id="design-system">
        <h2>Design system</h2>
        <p class="sub">Our sites and apps share one design system, <code>@spacecorps/components</code>: React components, renderers and widgets, with this site's colours and type taken from its tokens. Browse every component in Storybook.</p>
        <div class="cta" style="margin-top:0"><a class="btn" href="https://storybook.spacecorps.tech">Open Storybook</a><a class="btn o" href="https://github.com/SpaceCorps/storybook">Source and install notes</a></div>
      </section>
      <section>
        <h2>From the blog</h2>
        <p class="sub"><a href="/blog/spacecorps-2027.html">SpaceCorps 2027 is out</a> &middot; <a href="/blog/space3d-engine.html">Space3d: a Rust 3D engine built for AI agents</a> &middot; <a href="/blog/obop.html">obop: one business, one platform</a> &middot; <a href="/blog/getelec-spaceemit.html">47,000&times; faster electron emission</a></p>
      </section>
"""
page("index.html", "SpaceCorps | Games, engines and software in Rust", "SpaceCorps Technology AB builds the SpaceCorps 2027 space MMO, the Space3d engine, the obop open-source business platform and 24 open-source Rust command-line tools.", "/", index)

about = """      <div class="prose">
      <section class="hero" style="padding-bottom:0"><h1>About SpaceCorps</h1>
      <p class="lead">SpaceCorps Technology AB is a small Swedish software company. We build games, a game engine, business software and developer tools, mostly in Rust.</p></section>
      <h2>What we make</h2>
      <ul>
        <li><strong>SpaceCorps 2027</strong>, a free-to-play space MMO for macOS, Windows, Linux, Android and the browser.</li>
        <li><strong>Space3d</strong>, the Rust game engine the game runs on, also used for molecular simulation.</li>
        <li><strong>obop</strong>, an open-source business platform in early development.</li>
        <li><strong>Open-source tools</strong>: 24 Rust command-line tools and several libraries, all on <a href="https://github.com/SpaceCorps">GitHub</a>.</li>
        <li><strong>Custom physics software</strong> for researchers; see the <a href="/blog/getelec-spaceemit.html">spaceemit case study</a>.</li>
      </ul>
      <h2>How we work</h2>
      <p>A small team plus AI agents. Rust for services and performance-critical code, TypeScript on Vite+ for the web, everything shipped as containers. Every product site uses the same <a href="https://storybook.spacecorps.tech">design system</a>.</p>
      <h2>Elsewhere</h2>
      <p><a href="https://play.spacecorps.tech/">play.spacecorps.tech</a> game downloads &middot; <a href="https://web.spacecorps.xyz/">web.spacecorps.xyz</a> browser game &middot; <a href="https://space3d.dev/">space3d.dev</a> &middot; <a href="https://obop.dev/">obop.dev</a> &middot; <a href="https://storybook.spacecorps.tech">storybook.spacecorps.tech</a></p>
      </div>
"""
page("about.html", "About | SpaceCorps", "SpaceCorps Technology AB: a small Swedish company building games, the Space3d engine, business software and open-source Rust tools.", "/about.html", about, "c3")

contact = """      <div class="prose">
      <section class="hero" style="padding-bottom:0"><h1>Contact</h1>
      <p class="lead">The fastest way to reach us is where the work happens.</p></section>
      <h2>Questions, bugs and feature requests</h2>
      <ul>
        <li><strong>Game:</strong> <a href="https://github.com/SpaceCorps/play/issues">SpaceCorps 2027 issues</a> or our <a href="https://discord.gg/VjW67tkrTb">Discord</a>.</li>
        <li><strong>Engine:</strong> <a href="https://github.com/SpaceCorps/Space3d/issues">Space3d issues</a>.</li>
        <li><strong>obop:</strong> <a href="https://github.com/SpaceCorps/obop/issues">obop issues</a>.</li>
        <li><strong>Any other tool:</strong> open an issue in its repository on <a href="https://github.com/SpaceCorps">github.com/SpaceCorps</a>.</li>
      </ul>
      <h2>Security</h2>
      <p>Please report vulnerabilities privately with GitHub's &ldquo;Report a vulnerability&rdquo; on the affected repository (Security tab), not in a public issue.</p>
      <h2>Company</h2>
      <p>SpaceCorps Technology AB, Sweden.</p>
      </div>
"""
page("contact.html", "Contact | SpaceCorps", "How to reach SpaceCorps: issue trackers, Discord and private vulnerability reporting.", "/contact.html", contact, "c4")

privacy = """      <div class="prose">
      <section class="hero" style="padding-bottom:0"><h1>Privacy</h1>
      <p class="lead">This site collects nothing about you.</p></section>
      <h2>This website</h2>
      <p>spacecorps.tech sets no cookies, runs no analytics or trackers and loads no third-party scripts, fonts or images. Our web server keeps standard request logs (address, time, page) for security and operations, which we do not share.</p>
      <h2>The game</h2>
      <p>SpaceCorps 2027 has its own privacy policy, which covers the client, the game server and the download site: <a href="https://play.spacecorps.tech/privacy/">play.spacecorps.tech/privacy</a>. It is operated by SpaceCorps Technology AB.</p>
      <h2>Our command-line tools</h2>
      <p>They send no telemetry. API credentials you give a tool are stored in your operating system's keystore (macOS Keychain, Linux Secret Service or Windows DPAPI) and are sent only to the service the tool talks to. Each tool's repository documents what it contacts.</p>
      <h2>Contact</h2>
      <p>Questions about privacy: see <a href="/contact.html">Contact</a>.</p>
      </div>
"""
page("privacy.html", "Privacy | SpaceCorps", "What spacecorps.tech and SpaceCorps tools collect: nothing on this site, and links to the game policy.", "/privacy.html", privacy)

nf = """      <section class="hero"><h1>Page not found</h1><p class="lead">That page does not exist. Try the <a href="/">home page</a>, the <a href="/blog/">blog</a> or <a href="https://github.com/SpaceCorps">our repositories</a>.</p></section>
"""
page("404.html", "Not found | SpaceCorps", "Page not found.", "/404.html", nf)
