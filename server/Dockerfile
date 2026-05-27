FROM rust:1.94.0-alpine3.23 AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static

WORKDIR /root

COPY Cargo.toml Cargo.lock .cargo ./
COPY app/Cargo.toml ./app/
COPY health_check/Cargo.toml ./health_check/

RUN mkdir -p app/src && echo "fn main() {}" > app/src/main.rs
RUN mkdir -p health_check/src && echo "fn main() {}" > health_check/src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl && rm -rf app/src && rm -rf health_check/src

COPY app/src ./app/src
COPY health_check/src ./health_check/src
RUN touch app/src/main.rs && touch health_check/src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

### Additional compression with UPX
FROM alpine:3.19 AS compressor
RUN apk add --no-cache upx

COPY --from=builder /root/target/x86_64-unknown-linux-musl/release/app /app
RUN upx --best --lzma /app

COPY --from=builder /root/target/x86_64-unknown-linux-musl/release/health_check /health_check
RUN upx --best --lzma /health_check

### Final runtime image
FROM scratch
#FROM gcr.io/distroless/static:nonroot

#COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /app
COPY --from=compressor /app /app
COPY --from=compressor /health_check /health_check

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
#USER nonroot

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/health_check", "http://127.0.0.1:8000/api/v1/health_check"]

ENTRYPOINT ["/app"]
