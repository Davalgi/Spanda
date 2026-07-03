# Hosted Twin Cloud — Control Center with persisted twin snapshot store.
# Build from repo root: docker build -f deploy/twin-cloud-hosted/Dockerfile .
FROM rust:1-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build -p spanda --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/spanda /usr/local/bin/spanda
ENV SPANDA_CONTROL_CENTER_STATE_DIR=/var/lib/spanda
VOLUME /var/lib/spanda
EXPOSE 8080
EXPOSE 50051
ENTRYPOINT ["spanda", "control-center", "serve"]
CMD ["--bind", "0.0.0.0:8080"]
