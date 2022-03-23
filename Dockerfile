FROM rust:latest AS build-image

WORKDIR /auto-requeue
ADD . /auto-requeue

RUN apt update

RUN cargo build --release

EXPOSE 8000
RUN cargo run