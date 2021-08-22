use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_PIC32MX1XXFXXXB").is_some() {
            "src/pic32mx1xxfxxxb/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX1XXFXXXC").is_some() {
            "src/pic32mx1xxfxxxc/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX1XXFXXXD").is_some() {
            "src/pic32mx1xxfxxxd/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX2XXFXXXB").is_some() {
            "src/pic32mx2xxfxxxb/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX2XXFXXXC").is_some() {
            "src/pic32mx2xxfxxxc/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX2XXFXXXD").is_some() {
            "src/pic32mx2xxfxxxd/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX1X4FXXXB").is_some() {
            "src/pic32mx1x4fxxxb/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX1X4FXXXD").is_some() {
            "src/pic32mx1x4fxxxd/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX2X4FXXXB").is_some() {
            "src/pic32mx2x4fxxxb/device.x"
        } else if env::var_os("CARGO_FEATURE_PIC32MX2X4FXXXD").is_some() {
            "src/pic32mx2x4fxxxd/device.x"
        } else {
            panic!("No device features selected");
        };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
