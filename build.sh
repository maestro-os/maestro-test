#!/bin/sh

set -e

if [ -z "$ARCH" ]; then
	ARCH=i686-unknown-linux-musl
fi

# Build
cargo +nightly build --release -Zbuild-std --target $ARCH

# Create disk and filesystem
dd if=/dev/zero of=disk bs=1M count=1000
mkfs.ext2 disk

# Mount
mkdir -p mnt
sudo mount disk mnt

# Fill filesystem
cp target/$ARCH/release/maestro-test mnt/

# Cleanup
sudo umount mnt
