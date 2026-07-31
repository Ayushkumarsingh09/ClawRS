FROM node:22-alpine AS web
WORKDIR /web
COPY web/package.json web/package-lock.json* ./
RUN npm install --omit=dev=false
COPY web/ ./
RUN npm run build

FROM rust:1-bookworm AS builder
WORKDIR /app
COPY . .
COPY --from=web /web/dist ./web/dist
RUN cargo build --release -p clawrs-cli

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/clawrs /usr/local/bin/clawrs
ENV CLAWRS_LISTEN=0.0.0.0:8787
ENV CLAWRS_DATABASE_URL=sqlite:///data/clawrs.db?mode=rwc
ENV CLAWRS_STATIC_DIR=/web
COPY --from=web /web/dist /web
VOLUME /data
EXPOSE 8787
CMD ["clawrs", "serve", "--listen", "0.0.0.0:8787", "--static-dir", "/web"]
