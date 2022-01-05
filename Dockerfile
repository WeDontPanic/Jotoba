FROM rust:1.56-bullseye as build

WORKDIR app

COPY ./lib ./lib
COPY ./.git ./.git
COPY ./locales ./locales
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./scripts ./scripts
COPY ./jotoba_bin ./jotoba_bin
COPY ./LICENSE ./

RUN apt clean
RUN apt-get update --allow-releaseinfo-change -y
RUN apt upgrade -y
RUN apt install build-essential pkg-config libssl-dev libleptonica-dev libtesseract-dev clang tesseract-ocr-jpn -y

# Build your program for release
RUN cargo build --release

RUN mv target/release/jotoba .

FROM debian:bullseye

WORKDIR app

RUN apt-get update --allow-releaseinfo-change -y
RUN apt upgrade -y
RUN apt install build-essential pkg-config libssl-dev libleptonica-dev libtesseract-dev clang tesseract-ocr-jpn -y

COPY --from=build /app/jotoba .
COPY --from=build /app/locales ./locales

# Run the binary
CMD ["./jotoba","-s"]
