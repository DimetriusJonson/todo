FROM ubuntu AS builder

RUN apt-get update
RUN apt-get upgrade -y

RUN apt-get install -y curl clang gcc g++ zlib1g-dev libmpc-dev libmpfr-dev libgmp-dev git cmake pkg-config libssl-dev musl-dev build-essential

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s - -y

ENV PATH=/root/.cargo/bin:${PATH}

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /root

COPY reverse-proxy/Cargo.toml reverse-proxy/Cargo.lock reverse-proxy/.cargo ./
COPY reverse-proxy/proxy/Cargo.toml ./proxy/
COPY reverse-proxy/resource-proxy-pingora/Cargo.toml ./resource-proxy-pingora/
COPY reverse-proxy/resource-proxy-pingora/src ./resource-proxy-pingora/src

RUN mkdir -p proxy/src && echo "fn main() {}" > proxy/src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl && rm -rf proxy/src
#RUN cargo build --target x86_64-unknown-linux-musl && rm -rf proxy/src

COPY reverse-proxy/proxy/src ./proxy/src
RUN touch proxy/src/main.rs && cargo build --release --target x86_64-unknown-linux-musl
#RUN touch proxy/src/main.rs && cargo build --target x86_64-unknown-linux-musl

### Additional compression with UPX
FROM alpine:3.19 AS compressor
RUN apk add --no-cache upx

COPY --from=builder /root/target/x86_64-unknown-linux-musl/release/proxy /app
#COPY --from=builder /root/target/x86_64-unknown-linux-musl/debug/proxy /app
RUN upx --best --lzma /app

### Final runtime image
FROM scratch

COPY --from=compressor /app /app

COPY reverse-proxy/keys /root/keys
COPY client/dist /root/yew/www
#COPY todo-react/build /root/react/www
#COPY todo-svelte/build /root/svelte/www

#USER nonroot

ENTRYPOINT ["/app"]
