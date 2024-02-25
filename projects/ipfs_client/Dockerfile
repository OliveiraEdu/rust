FROM rust:1.68
RUN apt update
RUN apt install -y nano
WORKDIR /root

RUN git clone --depth 1 --single-branch https://github.com/OliveiraEdu/rust

WORKDIR /root/rust/projects/ipfs_client

RUN cargo run