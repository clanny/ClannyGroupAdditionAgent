FROM rust:1.61 AS builder
COPY . .
RUN cargo build --release

FROM debian:latest
WORKDIR /clanny

COPY --from=builder ./target/release/ClannyGroupAdditionAgent /bin/ClannyGroupAdditionAgent

RUN chmod +x /bin/ClannyGroupAdditionAgent

RUN \
  apt-get update && \
  apt-get install ca-certificates libssl-dev && \
  apt-get clean

CMD ["/bin/ClannyGroupAdditionAgent"]