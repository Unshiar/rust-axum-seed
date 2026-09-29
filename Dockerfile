# prepare dependencies stage
FROM rust:1.96-trixie AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build dependencies stage
FROM rust:1.96-trixie AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Здесь компилируются ТОЛЬКО зависимости. Этот слой будет браться из кэша GitHub, если вы не меняли Cargo.toml
RUN cargo chef cook --release --recipe-path recipe.json

# build stage
FROM rust:1.96-trixie AS builder
WORKDIR /app
COPY . .

COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release

# docker image stage
FROM debian:trixie-slim

RUN apt update && apt install -y ca-certificates curl tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/axum-app /usr/local/bin/axum-app
COPY ./entrypoint.sh ./

ENTRYPOINT ["./entrypoint.sh"]
