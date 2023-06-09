# Peripheral Access Crate for PIC32MX1/2xx

[![Crates.io](https://img.shields.io/crates/v/pic32mx2xx.svg)](https://crates.io/crates/pic32mx2xx)
[![docs.rs](https://img.shields.io/docsrs/pic32mx2xx.svg)](https://docs.rs/pic32mx2xx)

This crate contains peripheral access APIs for PIC32MX1xx/2xx devices including
some eXtreme Low Power (XLP) devices. The files in this repository were
generated using the tools [`edc2svd`] and [`svd2rust`].

[`edc2svd`]: https://github.com/kiffie/edc2svd
[`svd2rust`]: https://crates.io/crates/svd2rust

## Usage

This crates includes multiple modules for the individual variants of the MCU.
Features are used to selected the device, e.g.

```toml
[dependencies.pic32mx2xx]
version = "0.7.0"
features = ["pic32mx2xxfxxxb", "rt"]
```

## Supported Devices

| Module/Feature | Devices |
|:--------------:|:-------:|
| pic32mx1xxfxxxb | 28-pin PIC32MX1xx |
| pic32mx1xxfxxxc | 36-pin PIC32MX1xx |
| pic32mx1xxfxxxd | 44-pin PIC32MX1xx |
| pic32mx2xxfxxxb | 28-pin PIC32MX2xx |
| pic32mx2xxfxxxc | 36-pin PIC32MX2xx |
| pic32mx2xxfxxxd | 44-pin PIC32MX2xx |
| pic32mx1x4fxxxb | 28-pin PIC32MX1xx XLP |
| pic32mx1x4fxxxd | 44-pin PIC32MX1xx XLP |
| pic32mx2x4fxxxb | 28-pin PIC32MX2xx XLP |
| pic32mx2x4fxxxd | 44-pin PIC32MX2xx XLP |
