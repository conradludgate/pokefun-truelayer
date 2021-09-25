FROM lukemathwalker/cargo-chef:latest-rust-1.55.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM debian:buster-20210902-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/pokefun-truelayer /usr/local/bin

ARG PORT=8080
EXPOSE $PORT

ENTRYPOINT ["/usr/local/bin/pokefun-truelayer"]
