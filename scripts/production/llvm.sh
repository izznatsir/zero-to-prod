#!/bin/bash
################################################################################
# Install `clang`, `lldb`, and `lld` version 18 for Dockerfile builder layer.
################################################################################

set -eux

apt-get update
apt-get install -y software-properties-common

if [[ -z "`apt-key list 2> /dev/null | grep -i llvm`" ]]; then
    # Delete the key in the old format
    apt-key del AF4F7421
fi

if [[ ! -f /etc/apt/trusted.gpg.d/apt.llvm.org.asc ]]; then
    wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | tee /etc/apt/trusted.gpg.d/apt.llvm.org.asc
fi

add-apt-repository -y deb http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-18 main
# For some reason, the repository needs to be added twice to be registered properly
add-apt-repository -y deb http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-18 main
apt-get install -y clang-18 lldb-18 lld-18

ln -s /usr/bin/clang-18 /usr/bin/clang
ln -s /usr/bin/lldb-18 /usr/bin/lldb
ln -s /usr/bin/lld-18 /usr/bin/lld
