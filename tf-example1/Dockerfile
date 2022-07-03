FROM tensorflow/tensorflow

WORKDIR /home

# 必要なパッケージとRustをインストール
RUN apt update && apt install -y pkg-config libssl-dev && \
    curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH

# ビルド
COPY Cargo.toml Cargo.toml
COPY ./src ./src
RUN cargo build --release