FROM rust:latest as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/x402-polkadot-server /usr/local/bin/server

ENV RUST_LOG=info
EXPOSE 3000

CMD ["server"]
