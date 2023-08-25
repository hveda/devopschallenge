ARG RUST_VERSION=1.70.0
ARG APP_NAME=myapp

################################################################################
# Create a stage for building the application.
FROM rust:${RUST_VERSION} AS build-env
ARG APP_NAME
WORKDIR /app

COPY . .
# Build the rust app then copy the binary
RUN cargo install

CMD ["cargo","run"]
