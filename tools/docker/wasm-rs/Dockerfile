FROM rust:1.58.1

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack && \
    cargo install wasm-bindgen-cli

CMD ["wasm-pack"]