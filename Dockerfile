FROM rust:1 AS builder
WORKDIR /usr/src/statistics-4you
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/statistics-4you /usr/local/bin/statistics-4you
CMD ["statistics-4you"]
