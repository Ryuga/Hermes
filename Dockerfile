FROM ghcr.io/google/nsjail/nsjail:latest

USER root

RUN apt-get update && apt-get install -y \
    python3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /sandbox

ENTRYPOINT ["nsjail"]