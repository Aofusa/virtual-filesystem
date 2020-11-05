FROM rust:1.47-buster AS build-env
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /src
ADD . /src
RUN cargo test && cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build-env /usr/local/cargo/bin/virtual-filesystem /
WORKDIR /

CMD ["/virtual-filesystem"]