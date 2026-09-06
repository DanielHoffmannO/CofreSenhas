# Etapa de build: compila os binários com o toolchain completo do Rust.
FROM rust:1-slim-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Etapa final: imagem enxuta, só com os binários e libs de runtime.
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /usr/local/bin/server
COPY --from=builder /app/target/release/cofresenhas /usr/local/bin/cofresenhas

WORKDIR /data
EXPOSE 5000
ENTRYPOINT ["server"]
