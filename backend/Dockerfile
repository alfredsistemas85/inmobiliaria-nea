# Build stage
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app
COPY . .

# Build the binary (release)
ENV SQLX_OFFLINE=true
ENV CARGO_BUILD_JOBS=1
RUN cargo build --release --jobs 1

# Run stage
FROM debian:bookworm-slim

# Install necessary dependencies for Axum/Postgres/Rustls (e.g. ca-certificates)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/api /usr/local/bin/api
COPY --from=builder /usr/src/app/target/release/worker /usr/local/bin/worker
COPY --from=builder /usr/src/app/fonts ./fonts

EXPOSE 3000

CMD ["api"]
