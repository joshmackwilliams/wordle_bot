FROM rust:1.58.1-alpine
WORKDIR /usr/src/wordle_bot
COPY . .
RUN cargo install --path .
CMD ["wordle_bot"]