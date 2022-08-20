####################################################################################################
## Builder Image
####################################################################################################
FROM rust:1.62.0 AS builder

RUN update-ca-certificates

ENV USER=app
ENV UID=10001
ENV CARGO_HOME=/app/.cargo

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"

WORKDIR /app

COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

ENV CARGO_HOME=/app/.cargo

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

COPY --from=builder /app/target/release/app ./

USER app:app

CMD ["/app/app"]