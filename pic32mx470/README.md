# Peripheral Access Crate for PIC32MX330/350/370/430/450/470 microcontrollers

[![Crates.io](https://img.shields.io/crates/v/pic32mx470.svg)](https://crates.io/crates/pic32mx470)
[![docs.rs](https://img.shields.io/docsrs/pic32mx470.svg)](https://docs.rs/pic32mx470)

This Peripheral Access Crate (PAC) contains peripheral access APIs for
PIC32MX330/350/370/430/450/470 devices. The files in this repository were
generated using the tools [`edc2svd`] and [`svd2rust`].

[`edc2svd`]: https://github.com/kiffie/edc2svd
[`svd2rust`]: https://crates.io/crates/svd2rust

This PAC is not suitable for PIC32MX320/340/360/420/440/460 devices, which are a
different PIC32MX variant.

## Usage

This crates includes multiple modules for the individual variants of the MCU.
Features are used to selected the device, e.g.

```toml
[dependencies.pic32mx470]
version = "0.1.0"
features = ["pic32mx47xfxxxl", "rt"]
```

## Supported Devices

| Module/Feature  | Devices |
|:----------------|:--------|
| pic32mx37xfxxxl | PIC32MX330/350/370 (without USB)|
| pic32mx47xfxxxl | PIC32MX430/450/470 (with USB)

The PAC has been generated for high pin count device variants. Assuming that the
lower pin count variants have the same chips but with less pins bonded, the PAC
supports the lower pin count variants too.
