# Stage 1: Builder
FROM rust:1.81-bullseye as builder

WORKDIR /usr/src/simple-blog

COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

RUN rm src/main.rs

COPY src ./src

RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye

RUN apt-get update && apt-get install libpq5 -y

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

RUN mkdir images/

COPY assets/placeholder_avatar images/placeholder_avatar

COPY --from=builder /usr/src/simple-blog/target/release/simple-blog ./server

CMD ["./server"]
