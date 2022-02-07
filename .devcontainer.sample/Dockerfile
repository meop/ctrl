# base image
FROM rust:1-slim-bullseye

# for vs code remote tools
RUN apt update && \
    apt -y install \
        git \
        gpgconf \
        openssh-server
