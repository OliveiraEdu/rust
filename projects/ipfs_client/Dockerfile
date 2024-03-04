FROM rust:1.68
RUN apt update
RUN apt install -y nano

WORKDIR /root/Git

RUN git clone https://github.com/hyperledger/iroha.git --branch iroha2-lts

WORKDIR /root/Git/iroha

RUN cargo build -p iroha_client_cli --release

RUN cargo new my_client

COPY Cargo_iroha.toml /root/Git/iroha/Cargo.toml

COPY Cargo_my_client.toml /root/Git/iroha/my_client/Cargo.toml

WORKDIR /root/Git/iroha/my_client

#Notes
#It is necessary to update /root/Git/iroha/Cargo.toml to include "my_client" in the workspaces section
#It is necessary to update the dependencies of the /root/Git/iroha/my_client/Cargo.toml, copy from /root/Git/iroha/client_cli/Cargo.toml

#/projects

#RUN cargo new ipfs_client

#WORKDIR /root/projects/ipfs_client

#COPY ./configs/client_cli/config.json test_docker/

#RUN echo '{"comment":{"String": "Hello Meta!"}}' > test_docker/metadata.json

#COPY ./target/release/iroha_client_cli test_docker/
