FROM rust:1.42.0-slim-stretch

WORKDIR /var/app

ADD migrations migrations
ADD src src
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD diesel.toml diesel.toml
ADD Dockerfile Dockerfile

RUN apt update
RUN apt install -y libpq-dev libssl-dev pkg-config

RUN rustup update nightly
RUN cargo -v search --limit 0

RUN cargo +nightly build --release
CMD ./target/release/regular-dicers-backend
