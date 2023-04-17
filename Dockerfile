FROM rust:1-alpine3.17 AS builder
ADD . /build/
WORKDIR /build
RUN apk add --update-cache musl-dev
RUN apk add --update-cache pkgconfig
RUN apk add --update-cache openssl-dev
RUN cargo build --release

FROM alpine:3.17
COPY --from=builder /build/target/release/bdfd_ai_mod /app/
WORKDIR /app
ENV RUST_LOG=info
CMD ["./bdfd_ai_mod"]
