FROM lukemathwalker/cargo-chef:latest-rust-1.76.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin rustixapi

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY config /app/config
COPY --from=builder /app/target/release/rustixapi /usr/local/bin

EXPOSE 8080
ENV CARGO_PKG_NAME=$CARGO_PKG_NAME
ENV RUN_MODE=$RUN_MODE
ENV PORT=$PORT
ENTRYPOINT ["/usr/local/bin/rustixapi"]