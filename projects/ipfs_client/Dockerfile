FROM rust:1.68
RUN apt update
RUN apt install -y nano

WORKDIR /root/Git

RUN git clone https://github.com/OliveiraEdu/iroha.git --branch my_branch

WORKDIR /root/Git/iroha

RUN cargo build -p iroha_client_cli --release

WORKDIR /root/Git/iroha/my_client

RUN cargo run

