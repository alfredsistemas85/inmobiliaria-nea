# Build stage
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app
COPY . .

# Build the binary (release)
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

# Install necessary dependencies for Axum/Postgres/Rustls (e.g. ca-certificates)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend

EXPOSE 3000

CMD ["backend"]
