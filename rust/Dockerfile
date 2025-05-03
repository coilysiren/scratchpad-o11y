FROM rust:1.77-slim

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

EXPOSE 8001

CMD ["rust-server"]
