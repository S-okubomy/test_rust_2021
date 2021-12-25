FROM rust:1.53

ARG UID="1000"
ARG GID="1000"
ARG UNAME="okubo"
ARG GNAME="okubo"
RUN set -xe; apt-get update; apt-get install -y vim
RUN set -xe; groupadd -g ${GID} ${GNAME}; \
    useradd -m -g ${GID} -u ${UID} ${UNAME}
COPY dot.vimrc /home/${UNAME}/.vimrc
RUN set -xe; chown ${GNAME}:${UNAME} /home/${UNAME}/.vimrc
USER ${UNAME}

RUN rustup toolchain add nightly \
    && cargo +nightly install racer \
    && rustup component add rust-src \
    && rustup component add rustfmt-preview \
    && set -xe; curl https://raw.githubusercontent.com/Shougo/dein.vim/master/bin/installer.sh -o /tmp/installer.sh; sh /tmp/installer.sh ~/.vim/dein
WORKDIR /usr/src/myapp
COPY . .
