#!/bin/bash

# Will just dump the binary in the home directory as cannot guarantee any other directory
help="USAGE: deploy.sh [--release] <RPi IP> <binary>"

if [[ $# < 2 ]]
then
    echo $help
    exit 1
fi
out_dir="debug"

if [[ $1 == "--release" ]]
then
    release="--release"
    out_dir="release"
    shift
fi

binary=$2
rpi_ip=$1

set -ex
# Build for the Raspberry Pi
cargo build $release --target=arm-unknown-linux-gnueabihf

# Copy it across
scp target/arm-unknown-linux-gnueabihf/${out_dir}/${binary} pi@${rpi_ip}:
