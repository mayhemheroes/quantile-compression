FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /qcompress
WORKDIR /qcompress/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /qcompress/fuzz/target/x86_64-unknown-linux-gnu/release/qcompress-fuzz /