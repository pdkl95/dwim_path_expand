#!/bin/sh

export RUSTFLAGS="-Cembed-bitcode=yes"
exec xargo build --target x86_64-unknown-linux-gnu --release "$@"
