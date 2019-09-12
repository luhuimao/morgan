FROM debian:stretch

# JSON RPC port
EXPOSE 10099/tcp

# Install libssl
RUN apt update && \
    apt-get install -y libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY usr/bin /usr/bin/
ENTRYPOINT [ "/usr/bin/morgan-run.sh" ]
CMD [""]
