FROM rust:1.91 as builder

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/get_id_bot /usr/local/bin/get_id_bot

ENV RUST_LOG=info

CMD ["get_id_bot"]
