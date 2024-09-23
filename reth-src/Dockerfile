FROM rust:latest
RUN apt update && apt install -y cmake clang openssl

COPY exex-reth /reth
RUN cd /reth && make install-op && cd / && rm -rf /reth

# COPY exex-reth/target/release/op-reth /usr/local/cargo/bin/op-reth

WORKDIR /app

COPY exex-reth/scripts/* /app/scripts/

ENTRYPOINT op-reth
