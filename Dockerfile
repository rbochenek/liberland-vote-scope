FROM rust:1.85.1
WORKDIR /usr/src/liberland-vote-scope

COPY Cargo.toml Cargo.lock .
COPY artifacts artifacts
COPY patch patch
COPY src src

EXPOSE 8080

RUN cargo install --path .

CMD ["liberland-vote-scope"]
