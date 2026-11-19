"""Page shell shared by the generator in _pages.py (header, footer, head metadata)."""

HEAD = """<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{title}</title>
    <meta name="description" content="{desc}" />
    <link rel="canonical" href="https://spacecorps.tech{path}" />
    <meta property="og:title" content="{title}" />
    <meta property="og:description" content="{desc}" />
    <meta property="og:type" content="website" />
    <meta property="og:url" content="https://spacecorps.tech{path}" />
    <meta property="og:image" content="https://spacecorps.tech/og-image.png" />
    <meta name="twitter:card" content="summary_large_image" />
    <link rel="icon" href="/favicon.ico" sizes="32x32" />
    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
    <link rel="apple-touch-icon" href="/apple-touch-icon.png" />
    <link rel="alternate" type="text/markdown" href="/index.md" />
    <script src="/theme.js"></script>
    <link rel="stylesheet" href="/site.css" />
  </head>
  <body>
    <a class="skip" href="#main">Skip to content</a>
    <header class="nav"><div class="wrap">
      <a class="brand" href="/">Space<b>Corps</b></a>
      <a class="l" href="/#products">Products</a>
      <a class="l" href="/#open-source">Open source</a>
      <a class="l" href="/#design-system">Design system</a>
      <a class="l" href="/blog/">Blog</a>
      <a class="l" href="/about.html"{c3}>About</a>
      <a class="l" href="/contact.html"{c4}>Contact</a>
    </div></header>
    <main id="main" class="wrap">
"""

FOOT = """    </main>
    <footer><div class="wrap">
      <span class="sp">&copy; 2026 SpaceCorps Technology AB</span>
      <a href="https://github.com/SpaceCorps">GitHub</a>
      <a href="https://storybook.spacecorps.tech">Storybook</a>
      <a href="/privacy.html">Privacy</a>
      <a href="/contact.html">Contact</a>
      <a href="mailto:hello@spacecorpstechnology.com">hello@spacecorpstechnology.com</a>
    </div></footer>
    <script src="/wave.js" defer></script>
  </body>
</html>
"""


def page(file, title, desc, path, body, cur=None):
    cs = {k: (' aria-current="page"' if cur == k else "") for k in ("c3", "c4")}
    with open(file, "w", encoding="utf8", newline="\n") as f:
        f.write(HEAD.format(title=title, desc=desc, path=path, **cs) + body + FOOT)
