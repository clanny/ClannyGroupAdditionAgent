FROM alpine:latest

WORKDIR /clanny

COPY ./target/release/ClannyGroupAdditionAgent /clanny/ClannyGroupAdditionAgent

RUN chmod +x /clanny/ClannyGroupAdditionAgent

CMD ["/clanny/ClannyGroupAdditionAgent"]