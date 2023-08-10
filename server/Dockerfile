FROM rust:1.71.1-alpine3.17

RUN ["apk", "add", "musl-dev"]

COPY road-lurker /app/road-lurker

WORKDIR /app/road-lurker

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["cargo", "run"]