# spacecorps.tech

The company site of SpaceCorps Technology AB: <https://spacecorps.tech>. Static HTML built with Vite+ (`vp`) and pnpm, served by Caddy in a Docker container (Sliplane).

## Layout

| Path | What |
| --- | --- |
| `index.html`, `about.html`, `contact.html`, `privacy.html`, `404.html`, `blog/*.html` | The pages. **Generated** by `scripts/gen.mjs` (shell, home sections, inner pages) from `data/` and `content/blog/`; edit those, run `pnpm run gen`, commit both. Vite+ builds them as entry points. |
| `src/` | `main.js` (theme switch, CLI search, starfield, click wave) and `styles/` (`site.css`, vendored `tokens.css`). Bundled, minified and hashed by Vite+. |
| `data/` | `clis.json` (the 24 CLI directory entries) and `stats.json` (real GitHub numbers; refresh with `pnpm run stats`, needs the `gh` CLI). |
| `public/` | Copied to the site root as is: `img/` (game art as webp), `blog/media`, icons, og-image, and the **agent-facing files** below. `public/fonts/` is generated at build (Geist, from `@fontsource-variable`) and git-ignored. |
| `deploy/Caddyfile` | Caddy config: `/healthz`, text/markdown content types, caching, 404 page. |
| `Dockerfile` | Node 24 builds with `pnpm`, Caddy serves `dist/` on port 80. |
| `.github/workflows/docker.yml` | Builds the image on every PR and push to `main` and smoke-tests `/healthz`, `/`, `/llms.txt` and a 404. |
| `docs/REDIRECTS.md` | Plan for spacecorpstechnology.com, spacecorps.eu and spacecorps.online. |

## Design system

Colours, radii and type come from the SpaceCorps design system, `@spacecorps/components` ([SpaceCorps/storybook](https://github.com/SpaceCorps/storybook), Storybook at <https://storybook.spacecorps.tech>). `src/styles/tokens.css` is its `:root` and `.dark` token blocks, vendored; refresh with `pnpm run sync-tokens`. Dark mode follows the visitor's system setting (`src/main.js` toggles the `.dark` class; a manual choice is kept in `localStorage`).

The site does not install the React package: it is a static site with no React components, and the package is `restricted` on GitHub Packages, which would put a token into every build. If a page ever needs a component, follow "Consuming from another SpaceCorps site" in the storybook README (`.npmrc` scope plus a Docker build secret).

## Agent-facing files (single source: this repo)

All served from the site root and maintained only here: `llms.txt`, `index.md` (Markdown of the home page), `openapi.json` (the hosted spaceemit API), `robots.txt`, `sitemap.xml`. There is deliberately no MCP server card, sandbox or API-key service, and `.well-known/` holds only `security.txt`: none of those existed behind the old files (the advertised MCP endpoint returned 404). Add them back only when the service is real.

## Develop, build, run

```bash
pnpm install
pnpm run dev            # vp dev
pnpm run build          # fonts + gen + vp build -> dist/
pnpm run stats          # refresh data/stats.json from GitHub, then regenerate pages
docker build -t spacecorps-tech .
docker run --rm -p 8080:80 spacecorps-tech   # http://localhost:8080, health at /healthz
```

## Deploy (Sliplane)

Create a Sliplane service from this repository's `Dockerfile` (port 80, health check `/healthz`), then add the custom domain `spacecorps.tech` and the DNS record Sliplane shows. DNS and Sliplane configuration are made by hand; see `docs/REDIRECTS.md` for the other domains.

## Cleanup needed in the `play` repository (`SpaceCorps/play`)

Do not edit from here; these duplicate or contradict this site and should go from `play`:

- Duplicated "agent surface" files that describe a fictional MCP server, sandbox and API keys: `agent/`, `agent-instructions.md`, `agent-plugin.json`, `agent-plugins.json`, `ai-plugin.json`, `api/`, `api.json`, `ask`, `ask.json`, `auth.md`, `developers*`, `docs*`, `health*`, `keys*`, `plugin.json`, `pricing.md`, `sandbox*`, `server.json`, `v1/`, `v1.json`, `openapi.*`.
- In `llms.txt`, `llms-full.txt` and `index.md`: the `Model Context Protocol (MCP)` lines (`https://spaceemit-api.sliplane.app/mcp` returns 404, and that host is the spaceemit API, not the game), the sandbox, API-key and pricing links, and every `https://spacecorps.github.io/play/` canonical URL (becomes `https://play.spacecorps.tech/`). The generator is `scripts/marketing/sitedocs.py` in the game repository.
- `robots.txt` / `sitemap.xml` mentions of `spacecorps.github.io`.
- The root `robots.txt` of the old site listed `play/sitemap.xml`; this site's sitemap does not, because `play.spacecorps.tech` is its own host.
