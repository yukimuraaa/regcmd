FROM rust:1.58.1

WORKDIR /regcmd

RUN apt-get update -y && \
    apt-get install -y vim
RUN echo "alias ll='ls -al'" >> ~/.bashrc
