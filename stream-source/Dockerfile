FROM rust:alpine3.12 as builder
RUN apk add musl-dev
WORKDIR /str-source
### Cache the crates used
COPY src/cache.rs src/cache.rs
COPY Cargo.* ./
RUN cargo build --release --bin cache
###
COPY . .
RUN cargo build --release

FROM alpine:3.12
COPY --from=builder /str-source/target/release/str-source /bin/str-source
CMD ["str-source"]
