ARG RUST_VERSION=1.70.0
ARG APP_NAME=myapp

################################################################################
# Create a stage for building the application.
FROM rust:${RUST_VERSION} AS build-env
ARG APP_NAME
WORKDIR /app

COPY . .
# Build the rust app then copy the binary
RUN cargo build --release && cp ./target/release/$APP_NAME /bin/server

FROM gcr.io/distroless/cc
# Copy the executable from the "build-env" stage.
COPY --from=build-env /bin/server /
CMD ["./server"]
