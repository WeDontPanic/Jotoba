FROM rust:1.56-bullseye

WORKDIR app

COPY ./lib ./lib
COPY ./locales ./locales
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./scripts ./scripts
COPY ./jotoba_bin ./jotoba_bin
COPY ./LICENSE ./

RUN apt clean
RUN apt-get update --allow-releaseinfo-change
RUN apt upgrade
RUN apt install build-essential pkg-config libssl-dev libleptonica-dev libtesseract-dev clang tesseract-ocr-jpn -y

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/jotoba","-s"]
