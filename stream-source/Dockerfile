FROM rust:alpine3.12 as builder
WORKDIR /str-source
COPY . .
RUN cargo build --release

FROM alpine:3.12
COPY --from=builder /str-source/target/release/str-source /bin/str-source
CMD ["str-source"]
