FROM rust:1 AS builder
ADD . /build/
WORKDIR /build
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update
RUN apt-get install ca-certificates -y
update-ca-certificates
COPY --from=builder /build/target/release/bdfd_ai_mod /app/
WORKDIR /app
ENV RUST_LOG=info
CMD ["./bdfd_ai_mod"]

