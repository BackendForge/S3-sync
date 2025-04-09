FROM rust:latest as build
WORKDIR /app
COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo build --release

# FROM gcr.io/distroless/static:latest as base
FROM gcr.io/distroless/cc:latest as base
# Copy binary files
COPY --from=build /app/target/release/rados-list /app/rados-list
# Setup working directory
WORKDIR /app
COPY ./.env /app/.env
# Run the application
USER nobody
ENTRYPOINT ["/app/rados-list"]