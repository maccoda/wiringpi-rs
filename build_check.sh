#!/bin/bash

# Whilst we struggle to get CI set up for this will just this to ensure
# everything keeps working
set -ex

cargo build --target=arm-unknown-linux-gnueabihf

cargo build --target=arm-unknown-linux-gnueabihf --examples
