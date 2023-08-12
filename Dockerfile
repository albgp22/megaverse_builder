FROM rust:latest
COPY ./ ./
RUN cargo build --release
ENV RUST_LOG=debug
ENTRYPOINT ["./target/release/megaverse-builder"]