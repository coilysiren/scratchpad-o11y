FROM rust:1.86 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/rust-server /usr/src/app/rust-server
EXPOSE 8001
CMD ["./rust-server"]
