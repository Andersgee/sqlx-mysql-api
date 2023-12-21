FROM rust:1.74-slim-bullseye as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/sqlx-mysql-api /usr/local/bin/myapp
EXPOSE 3306 80
CMD ["myapp"]
