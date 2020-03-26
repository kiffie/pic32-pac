#!/bin/sh -x

EDC=PIC32MX270F256B.PIC
SVD=PIC32MX270F256B.svd

edc2svd $EDC $SVD
python3 ../svdpatch.py svdpatch.yaml
svd2rust --target none -i $SVD.patched
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
python3 ../svd2devicex.py -o device.x $SVD.patched
