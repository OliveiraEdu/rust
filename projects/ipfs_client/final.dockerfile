# final.dockerfile
FROM ubuntu:latest

RUN mkdir /root/iroha/test_docker

# Copy application binary from builder image
COPY --from=builder /root/iroha/tmp/iroha_client_cli /root/iroha/test_docker

COPY --from=builder /root/iroha/tmp/config.json /root/iroha/test_docker

