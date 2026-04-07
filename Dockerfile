# Build stage
FROM rust:1.83-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY static/ static/
RUN cargo build --release --bin web

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/web /usr/local/bin/web
COPY --from=builder /app/static/ /app/static/
WORKDIR /app
ENV PORT=8080
EXPOSE 8080
CMD ["web"]
