FROM rust:1.67

WORKDIR /root

RUN git clone https://github.com/hyperledger/iroha.git --branch iroha2-lts

WORKDIR /root/iroha

RUN cargo build -p iroha_client_cli --release

#/projects

#RUN cargo new ipfs_client

#WORKDIR /root/projects/ipfs_client
