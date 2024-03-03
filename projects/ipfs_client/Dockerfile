FROM rust:1.68
RUN apt update
RUN apt install -y nano
WORKDIR /root

<<<<<<< HEAD
WORKDIR /root

RUN git clone https://github.com/hyperledger/iroha.git --branch iroha2-lts

WORKDIR /root/iroha

RUN cargo build -p iroha_client_cli --release

#/projects

#RUN cargo new ipfs_client

#WORKDIR /root/projects/ipfs_client
=======
RUN git clone --depth 1 --single-branch https://github.com/OliveiraEdu/rust

WORKDIR /root/rust/projects/ipfs_client

RUN cargo run
>>>>>>> 9a7d3979dc8fda95a2bdedbfe331e3f7aba77a88
