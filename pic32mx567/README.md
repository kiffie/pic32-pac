# Peripheral Access Crate for PIC32MX5xx/6xx/7xx

[![Crates.io](https://img.shields.io/crates/v/pic32mx567.svg)](https://crates.io/crates/pic32mx567)
[![docs.rs](https://img.shields.io/docsrs/pic32mx567.svg)](https://docs.rs/pic32mx567)

This crate contains peripheral access APIs for PIC32MX5xx/6xx/7xx devices. The
files in this repository were generated using the tools [`edc2svd`] and
[`svd2rust`].

[`edc2svd`]: https://github.com/kiffie/edc2svd
[`svd2rust`]: https://crates.io/crates/svd2rust

## Usage

This crates includes multiple modules for the individual variants of the MCU.
Features are used to selected the device, e.g.

```toml
[dependencies.pic32mx567]
version = "0.1.0"
features = ["pic32mx695fxxxl", "rt"]
```

## Supported Devices

| Module/Feature | Devices |
|:--------------:|:-------:|
| pic32mx695fxxxl | PIC32MX6xx with USB and Ethernet |
