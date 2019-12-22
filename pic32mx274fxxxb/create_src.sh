#!/bin/sh

EDCPATH=/opt/microchip/mplabx/v5.25/packs/Microchip/PIC32MX_DFP/1.0.53/edc

edc2svd $EDCPATH/PIC32MX274F256B.PIC PIC32MX274F256B.svd
python3 svdpatch.py svdpatch.yaml
svd2rust --target none -i PIC32MX274F256B.svd.patched
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
