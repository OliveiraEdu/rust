# builder.dockerfile
FROM rust:1.68

RUN apt update

RUN apt install -y nano

WORKDIR /root

RUN git clone https://github.com/hyperledger/iroha.git --branch iroha2-lts

WORKDIR /root/iroha

RUN cargo build -p iroha_client_cli --release

RUN mkdir /root/iroha/tmp

# Copy necessary files for final image
COPY /root/iroha/configs/client_cli/config.json /root/iroha/tmp

COPY /root/iroha/target/release/iroha_client_cli /root/iroha/tmp
