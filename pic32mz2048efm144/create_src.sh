edc2svd PIC32MZ2048EFM144.PIC PIC32MZ2048EFM144.svd
svd2rust --target=mips -i PIC32MZ2048EFM144.svd

mv lib.rs src/lib.rs

cargo fmt
cargo doc