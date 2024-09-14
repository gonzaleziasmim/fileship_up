FROM rust:1.58 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/file_upload_service /usr/local/bin/file_upload_service

WORKDIR /app
EXPOSE 3000

CMD ["file_upload_service"]
