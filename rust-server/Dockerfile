FROM rust:1.86

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

EXPOSE 8001

CMD ["rust-server"]
