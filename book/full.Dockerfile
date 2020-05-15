FROM rust:alpine3.11
WORKDIR /usr/src/code-library-book
COPY . .
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add gcc musl-dev
RUN cargo install --path .
CMD ["book"]