FROM rust:bookworm as builder
WORKDIR /usr/src/webserver-demo
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/webserver-demo /usr/local/bin/
CMD ["webserver-demo"]
