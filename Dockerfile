FROM rustlang/rust:nightly as builder
WORKDIR /app/src
RUN USER=root cargo new --bin my_bevy_game
RUN apt update && apt install -y libasound2-dev  pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev
COPY Cargo.toml Cargo.lock ./my_bevy_game/

WORKDIR /app/src/my_bevy_game
RUN cargo build --release

COPY ./ ./
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
RUN apt update \
    && apt install -y libasound2 openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

EXPOSE 80 443

COPY --from=builder /app/src/my_bevy_game/target/release/my_bevy_game  ./

CMD ["/app/my_bevy_game"]