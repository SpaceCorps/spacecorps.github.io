# Domain and redirect plan

Plan only. Nothing here has been applied: no DNS or Sliplane settings were changed.

## Target state

| Host | Serves |
| --- | --- |
| `spacecorps.tech` | **this site** (canonical company home) |
| `www.spacecorps.tech` | 301 to `https://spacecorps.tech{uri}` |
| `play.spacecorps.tech` | game downloads (the `play` repo's static site) |
| `storybook.spacecorps.tech` | Storybook (SpaceCorps/storybook) |
| `web.spacecorps.xyz`, `spacecorps.xyz` | browser build of the game (unchanged) |
| `obop.dev`, `space3d.dev` | product sites (unchanged) |
| `spacecorpstechnology.com`, `www.` | 301 to `https://spacecorps.tech{uri}` (old site, title "SpaceCorps Technology") |
| `spacecorps.eu`, `spacecorps.online`, and their `www.` | 301 to `https://spacecorps.tech{uri}` |
| `spacecorps.github.io` | old GitHub Pages site; see below |

## Steps (in order)

1. **Sliplane**: deploy this repo's `Dockerfile` as a service (port 80, health check `/healthz`). Check it on the temporary `*.sliplane.app` URL first.
2. **DNS, `spacecorps.tech`**: add the custom domain in Sliplane, then create the record it shows (CNAME for `www`; for the apex, the A/ALIAS record Sliplane specifies). Wait for the certificate; `https://spacecorps.tech/healthz` must return `ok`. Same procedure for `play.` and `storybook.` (`storybook` is described in the storybook README).
3. **Redirect domains**: either (a) give each domain's DNS to Sliplane and add all four as custom domains of a second tiny service running the redirect Caddy below, or (b) use the registrar's / Cloudflare's own redirect rule if the zone is on Cloudflare (a single "redirect all, keep path, 301" rule per domain). (b) needs no extra container; (a) keeps everything in one place. Recommendation: (b) for the three domains that are only redirects.
4. **Last**: switch the old content off only after step 2 works, and keep the redirects for at least a year.

## Redirect-only Caddyfile (option a)

```caddyfile
{
	auto_https off
	admin off
}

:80 {
	@health path /healthz
	respond @health "ok" 200
	redir https://spacecorps.tech{uri} permanent
}
```

Run it as `caddy:2-alpine` with that file at `/etc/caddy/Caddyfile`. TLS is terminated by Sliplane, so every hostname must be added as a custom domain of this service for certificates to be issued. Add `www.spacecorps.tech` here as well.

## `spacecorps.github.io`

GitHub Pages cannot redirect by HTTP status. This PR removes the Pages workflow, so the last deployed old version stays online until Pages is disabled. Options, after `spacecorps.tech` is live: (1) leave it (project pages such as `spacecorps.github.io/Sliplane-Cli/` and `/play/` keep working, and they are linked from many places); (2) replace `index.html` on a small branch with a meta-refresh and `rel=canonical` to `https://spacecorps.tech/`. Do **not** set `spacecorps.tech` as the Pages custom domain: that would move every project page (`/Sliplane-Cli/`, `/open-glass/` ...) under `spacecorps.tech/<repo>/`.

## Checks after cutover

```bash
for h in spacecorpstechnology.com www.spacecorpstechnology.com spacecorps.eu spacecorps.online www.spacecorps.tech; do
  curl -sI "https://$h/about.html" | grep -iE '^(HTTP|location)'
done    # expect 301 and Location: https://spacecorps.tech/about.html
curl -s https://spacecorps.tech/healthz   # ok
```

Also update: the `homepage` field of the GitHub org and of the repositories that point at `spacecorps.github.io` or `spacecorpstechnology.com` (for example `SpaceCorps/web`), Google Search Console (add `spacecorps.tech`, submit the sitemap, use "Change of address" for spacecorpstechnology.com).
