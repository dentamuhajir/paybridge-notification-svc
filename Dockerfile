FROM rust:1.84 AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Build your actual project
COPY . .
RUN cargo build --release

# Final lightweight image
FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/insurance_claim_management_system /app/app

EXPOSE 8081
CMD ["./app"]
