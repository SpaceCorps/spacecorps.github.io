# spacecorps.tech status service: samples the game's public server list every minute into SQLite and serves
# /api/status/series. Build context is this folder (status/).
#   docker build -t spacecorps-status status/
#   docker run --rm -p 8090:8090 -v status-data:/data spacecorps-status      # http://localhost:8090/healthz
FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev gcc
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked --bin spacecorps-status

FROM alpine:3
# su-exec: the entrypoint makes the (root-owned) volume writable, then runs the service as an unprivileged user.
RUN apk add --no-cache su-exec && adduser -D -u 10001 status && mkdir /data && chown status /data
COPY --from=build /src/target/release/spacecorps-status /usr/local/bin/spacecorps-status
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
ENV STATUS_DB=/data/status.db STATUS_ADDR=0.0.0.0:8090
VOLUME /data
EXPOSE 8090
HEALTHCHECK --interval=30s --timeout=3s CMD wget -qO- http://127.0.0.1:8090/healthz >/dev/null || exit 1
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
