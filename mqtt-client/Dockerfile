FROM rust:1.78-slim
# cache-buster 2024-06-09-19:47
WORKDIR /app
COPY Cargo.toml .
RUN date +%s > /tmp/bust
COPY src ./src
RUN cargo build --release
CMD ["./target/release/mqtt-client"]
