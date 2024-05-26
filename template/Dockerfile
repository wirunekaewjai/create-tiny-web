# Build stage
FROM rust:bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Final run stage
FROM gcr.io/distroless/cc-debian12 AS runner

WORKDIR /app

COPY --from=builder /app/target/release/htmx-csr /app/htmx-csr
COPY --from=builder /app/assets /app/assets
COPY --from=builder /app/public /app/public

CMD ["/app/htmx-csr"]