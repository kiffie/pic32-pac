# Peripheral Access Crate for PIC32MX2xx

The files in this repository were generated using the tools [`edc2svd`] and [`svd2rust`].

[`edc2svd`]: https://github.com/kiffie/edc2svd
[`svd2rust`]: https://crates.io/crates/svd2rust

## Usage

This crates includes multiple modules for the individual variants of the MCU.
Features are used to selected the device, e.g.

```toml
[dependencies.pic32mx2xx]
version = "0.2.0"
features = ["pic32mx2xxfxxxb", "rt"]
```

## Supported Devices

| Module/Feature | Devices |
|:--------------:|:-------:|
| pic32mx2xxfxxxb | 28-pin PIC32MX2xx |
| pic32mx2xxfxxxc | 36-pin PIC32MX2xx |
| pic32mx2xxfxxxd | 44-pin PIC32MX2xx |
