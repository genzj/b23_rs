# Build stage
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app
COPY . .

# Build for release
RUN cargo build --release

# Final stage
FROM gcr.io/distroless/cc-debian12

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/b23_rs /app/b23_rs

# Expose the port the app runs on
EXPOSE 3000

# Run the binary
CMD ["/app/b23_rs"]
