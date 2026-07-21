FROM rust:1.79-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM python:3.12-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust2cython /usr/local/bin/rust2cython
COPY playground/ ./playground/
ENV RUST2CYTHON_BIN=/usr/local/bin/rust2cython
ENV PORT=8080
EXPOSE 8080
CMD ["python3", "playground/server.py"]
