FROM rust:slim AS build-image

WORKDIR /auto-requeue
ADD . /auto-requeue

RUN apt update

RUN cargo build --release

FROM rust:latest AS final-image

COPY --from=build-image /auto-requeue/target/release/auto-requeue .
COPY --from=build-image /auto-requeue/Cargo.toml .

EXPOSE 4231
CMD ["./auto-requeue"]