# Domain and redirect setup

What is live, and what is still open.

## State

| Host | Serves | How |
| --- | --- | --- |
| `spacecorps.tech` | this site | Sliplane service `spacecorps-site` (this repo's Dockerfile, port 80, health check `/healthz`) |
| `www.spacecorps.tech` | 301 to `https://spacecorps.tech{uri}` | `deploy/Caddyfile` (needs the host added as a custom domain on the service) |
| `spacecorpstechnology.com`, `www.` | 301 to `https://spacecorps.tech{uri}`, path and query kept | Custom domains on `spacecorps-site`, redirected by `deploy/Caddyfile`. **Google Workspace MX records are kept untouched**: only the web records (apex and `www`) point at Sliplane. |
| `spacecorps.eu`, `spacecorps.online` | 301 to `https://spacecorps.tech` | Namecheap "URL Redirect" records (301, **http only**: there is no certificate, so `https://spacecorps.eu` shows a certificate error until these move to a Caddy-served custom domain) |
| `play.spacecorps.tech` | game downloads (`play` repo) | separate service |
| `storybook.spacecorps.tech` | Storybook | separate service, see the storybook README |
| `web.spacecorps.xyz`, `obop.dev`, `space3d.dev` | unchanged | |
| `spacecorps.github.io` | old GitHub Pages site | see below |

Company email is on `@spacecorpstechnology.com` (Google Workspace): `hello@` for general contact, `security@` for vulnerability reports (also published in `/.well-known/security.txt`). That is why the domain's MX must stay as it is.

## The Caddy rules

`deploy/Caddyfile` uses ordered, mutually exclusive `handle` blocks: `/healthz` first (answers `ok` for every host, so health checks never get redirected), then the two redirect host matchers, then the static site. Check locally:

```bash
docker build -t spacecorps-tech . && docker run -d --name t -p 8081:80 spacecorps-tech
curl -sI -H 'Host: spacecorpstechnology.com' 'localhost:8081/about.html?x=1'   # 301, Location: https://spacecorps.tech/about.html?x=1
curl -s  -H 'Host: spacecorpstechnology.com' localhost:8081/healthz              # ok
```

## Open

- `https://spacecorps.eu` and `https://spacecorps.online` (https) do not redirect. Fix: add both (and `www.`) as custom domains on `spacecorps-site` and extend the `@oldcompany` matcher in the Caddyfile, then drop the Namecheap redirects.
- `spacecorps.github.io`: GitHub Pages cannot send HTTP redirects. Project pages such as `/Sliplane-Cli/` and `/play/` keep working there, so leave the org site as is, or replace its index with a meta refresh. Do **not** set `spacecorps.tech` as its Pages custom domain: that would move every project page under `spacecorps.tech/<repo>/`.
- Update the `homepage` field of repositories that still point at `spacecorps.github.io` or `spacecorpstechnology.com` (for example `SpaceCorps/web`), add `spacecorps.tech` to Google Search Console and use "Change of address" for spacecorpstechnology.com. Keep the redirects for at least a year.

## After a change

```bash
for h in spacecorpstechnology.com www.spacecorpstechnology.com www.spacecorps.tech; do
  curl -sI "https://$h/about.html" | grep -iE '^(HTTP|location)'
done    # expect 301 and Location: https://spacecorps.tech/about.html
curl -s https://spacecorps.tech/healthz   # ok
```
