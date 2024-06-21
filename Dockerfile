FROM rust:1 AS builder
ADD . /build/
WORKDIR /build
RUN cargo build --release

FROM ubuntu:latest
RUN apt-get update
RUN apt-get upgrade
RUN apt-get install -y ca-certificates

COPY --from=builder /build/target/release/bdfd_ai_mod /app/
WORKDIR /app
ENV RUST_LOG=info
CMD ["./bdfd_ai_mod"]
