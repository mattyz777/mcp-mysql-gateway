FROM rust:1.96-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM alpine:3.19

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/app /app/app
COPY config.toml /app/config.toml

EXPOSE 5111

CMD ["./app"]
