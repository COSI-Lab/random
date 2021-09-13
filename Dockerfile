FROM rust AS builder
WORKDIR /random

# Build dependencies
RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

# Prepare build
COPY src src
RUN touch src/main.rs

# Build release
RUN cargo build --release

# Run binary
FROM rust:slim
WORKDIR /app
COPY --from=builder /random/target/release/random random
COPY static static
CMD ["./random"]
