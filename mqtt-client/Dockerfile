FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/mqtt-client /usr/local/bin/mqtt-client
CMD ["mqtt-client"]
