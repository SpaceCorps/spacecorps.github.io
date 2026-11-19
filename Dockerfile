# spacecorps.tech: static site, built with Vite+ (vp) and pnpm, served by Caddy.
#   docker build -t spacecorps-tech .
#   docker run --rm -p 8080:80 spacecorps-tech      # http://localhost:8080, health at /healthz
FROM node:24-alpine AS build
RUN corepack enable
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .
RUN pnpm run build

FROM caddy:2-alpine
COPY deploy/Caddyfile /etc/caddy/Caddyfile
COPY --from=build /app/dist /srv
EXPOSE 80
HEALTHCHECK --interval=30s --timeout=3s CMD wget -qO- http://127.0.0.1/healthz >/dev/null || exit 1
