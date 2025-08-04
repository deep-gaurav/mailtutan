FROM rust:alpine as builder

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app
COPY ./ ./ 

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/mailtutan

FROM debian:bullseye-slim

RUN apt-get update &&     apt-get install -y libssl1.1 ca-certificates &&     apt-get clean

RUN mkdir /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/mailtutan /app/server

WORKDIR /app

ENTRYPOINT ["/bin/sh", "-c", "/app/server --smtp-relay-server $SMTP_SERVER --smtp-relay-server-username $SMTP_USER --smtp-relay-server-password $SMTP_PASS --smtp-auth-username $SMTP_LOGIN_USER --smtp-auth-password $SMTP_AUTH_PASS"]