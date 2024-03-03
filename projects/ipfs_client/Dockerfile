FROM rust:1.68
RUN apt update
RUN apt install -y nano
WORKDIR /root

WORKDIR /root

RUN git clone https://github.com/hyperledger/iroha.git --branch iroha2-lts

WORKDIR /root/iroha

RUN cargo build -p iroha_client_cli --release

#/projects

#RUN cargo new ipfs_client

#WORKDIR /root/projects/ipfs_client

#COPY ./configs/client_cli/config.json test_docker/

#RUN echo '{"comment":{"String": "Hello Meta!"}}' > test_docker/metadata.json

#COPY ./target/release/iroha_client_cli test_docker/
