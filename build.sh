#!/bin/sh

set -e

if [ -z "$TARGET" ]; then
	export TARGET=i686-unknown-linux-musl
fi

# Build
cargo build --release -Zbuild-std --target "$TARGET"

# Create disk and filesystem
dd if=/dev/zero of=disk bs=1M count=1024
mkfs.ext2 disk

# Mount
mkdir -p mnt
sudo mount disk mnt

# Fill filesystem
cp "target/$TARGET/release/maestro-test" mnt/

# Cleanup
sudo umount mnt
