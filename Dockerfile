FROM rust:latest AS builder

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine:latest
RUN apk --no-cache add gcompat

COPY --from=builder /usr/local/cargo/bin/wimc /usr/local/bin/wimc
CMD ["/usr/local/bin/wimc"]
EXPOSE 8080