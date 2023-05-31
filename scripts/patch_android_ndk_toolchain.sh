#!/bin/bash

# this patch can go away once this fix bubbles up
# through its consuming packages:
# https://github.com/alexcrichton/openssl-src-rs/pull/173
d="$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin"

if [ -d ${d} ]
then
    declare -a bins=("ar" "ranlib")
    
    for l in "${bins[@]}"
    do
        ln -sf ${d}/llvm-${l} ${d}/aarch64-linux-android-${l}
    done
fi
