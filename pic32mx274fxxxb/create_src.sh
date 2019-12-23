#!/bin/sh

edc2svd PIC32MX274F256B.PIC PIC32MX274F256B.svd
python3 svdpatch.py svdpatch.yaml
svd2rust --target none -i PIC32MX274F256B.svd.patched
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
