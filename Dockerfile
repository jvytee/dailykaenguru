FROM docker.io/library/rust:1.52-alpine3.13 AS builder

ARG src=/usr/local/src/dailykaenguru
RUN apk update && apk add build-base openssl-dev pkgconf

COPY Cargo.toml Cargo.lock $src/
COPY src $src/src
RUN cargo install --path $src

FROM docker.io/library/alpine:3.13

ENV RUST_LOG=info
ENV DAILYKAENGURU_DATA=/var/lib/dailykaenguru
ENV DAILYKAENGURU_DELIVERY=09:30
ENV DAILYKAENGURU_TOKEN=

RUN apk update && apk add ca-certificates openssl
RUN addgroup -S dailykaenguru && adduser -G dailykaenguru -S -H dailykaenguru

COPY --from=builder /usr/local/cargo/bin/dailykaenguru /usr/local/bin/dailykaenguru

USER dailykaenguru
CMD dailykaenguru
