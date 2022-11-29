FROM rust:1.61 AS builder
COPY . .
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /clanny

COPY --from=builder ./target/release/ClannyGroupAdditionAgent /bin/ClannyGroupAdditionAgent

RUN chmod +x /bin/ClannyGroupAdditionAgent

RUN apt-get update && apt-get install -y libssl-dev

CMD ["/bin/ClannyGroupAdditionAgent"]