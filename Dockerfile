FROM rust:1.73-slim

WORKDIR /root/app

COPY . /root/app/

RUN cargo build
