#!/bin/bash
set -e

echo "Starting Release Build"


echo "Compiling release binary"
cargo build --release


echo "Stripping binary"
strip target/release/hyperfetch


VERSION=$(grep '^version =' Cargo.toml | cut -d '"' -f 2)
ARCHIVE_NAME="hyperfetch-linux-x86_64-v${VERSION}.tar.gz"

echo "Creating archive: $ARCHIVE_NAME"
tar -czvf "$ARCHIVE_NAME" -C target/release hyperfetch

echo "Build Complete"
echo "------------------------------------------------"
echo "You can now distribute: $ARCHIVE_NAME"
echo "File size: $(du -h "$ARCHIVE_NAME" | cut -f1)"
echo "------------------------------------------------"
