#!/bin/bash

# Rust prebuilt libstd is targeting Android NDK 22
# Android NDK 23+ uses libunwind now, not libgcc
# so we need to either:
# 1. wait for Rust to come up with solution for prebuilt libstd (so far, none)
# 2. rebuild libstd from source (would need nighly build of Rust, which includes source, and do extra work)
# https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
# 3. force libgcc to really use libunwind under the hood
# https://github.com/rust-lang/rust/pull/85806#issuecomment-1096266946

# would rather be specific than to blindly patch whatever folder patterns are found
# because hopefully Rust stdlib upgrades to newer compiler by default soon
[ -z "${ANDROID_NDK_VERSION}" ] && ANDROID_NDK_VERSION="25.1.8937393"
[ -z "${ANDROID_NDK_CLANG_VERSION}" ] && ANDROID_NDK_CLANG_VERSION="14.0.6"

declare -a hosts=("darwin" "linux")

for h in "${hosts[@]}"
do
    if [ -z "${ANDROID_NDK}" ]
    then
        [ ${h} == "darwin" ] && ANDROID_NDK="${HOME}/Library/Android/sdk/ndk/${ANDROID_NDK_VERSION}"
        [ ${h} == "linux" ] && ANDROID_NDK="${HOME}/Android/Sdk/ndk/${ANDROID_NDK_VERSION}"
    fi

    d="${ANDROID_NDK}/toolchains/llvm/prebuilt/${h}-x86_64/lib64/clang/${ANDROID_NDK_CLANG_VERSION}/lib/linux/aarch64"
    if [ -d ${d} ]
    then
        p="${d}/libgcc.a"
        echo "generating: ${p}"
        echo "INPUT(-lunwind)" > "${p}"
    fi
done
