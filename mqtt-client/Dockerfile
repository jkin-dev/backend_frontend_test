FROM rust:1.75 AS builder

WORKDIR /app

# Copy only Rust files
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/mqtt-client /usr/local/bin/mqtt-client
CMD ["mqtt-client"]
