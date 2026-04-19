FROM rust:1.88-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        bash \
        ca-certificates \
        libssl-dev \
        pkg-config \
        postgresql-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

CMD ["bash"]