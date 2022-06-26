FROM rust:1.59-buster

RUN apt-get update && \
    apt-get -y install git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch

# ARG UID="1000"
# ARG GID="1000"
# ARG UNAME="okubo"
# ARG GNAME="okubo"
# RUN set -xe; groupadd -g ${GID} ${GNAME}; \
#     useradd -m -g ${GID} -u ${UID} ${UNAME}
# USER ${UNAME}