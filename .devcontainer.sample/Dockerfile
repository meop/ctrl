# base image
FROM rust:1-slim-bullseye

# for cross compiling
RUN rustup target install \
    x86_64-apple-darwin \
    x86_64-pc-windows-gnu

# for vs code remote tools
RUN apt update && \
    apt -y install \
        git \
        gpgconf \
        openssh-server

# g++-mingw-w64-x86-64
# binutils-mingw-w64-x86-64
