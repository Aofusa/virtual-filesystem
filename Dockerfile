FROM rust:1.47-buster AS build-env
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /work
ADD . /work
RUN cargo test && cargo rustc --release --bin cli --target x86_64-unknown-linux-musl -- -C opt-level=s -C link-args=-Wl,-x,-S

FROM scratch
COPY --from=build-env /work/target/x86_64-unknown-linux-musl/release/cli /virtual-filesystem
CMD ["/virtual-filesystem"]