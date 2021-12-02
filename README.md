# knx_rs

[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![crates.io](https://meritbadge.herokuapp.com/knx_rs)](https://crates.io/crates/knx_rs)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fuwearzt%2Fknx_rs.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fuwearzt%2Fknx_rs?ref=badge_shield)

The `knx_rs` crate implements a
[KNX](https://en.wikipedia.org/wiki/KNX_(standard)) Library able to communicate over IP (Multicast) and Serial.

It is used to create some user specific Actors and Sensors on the twisted
wire bus (TP1).

## Hardware

[TP-UART 2 Evaluation Board](http://www.opternus.com/uploads/media/PCBA_UP117-12_datasheet_v5_2012-05-30.pdf)
[TPUART2](http://www.opternus.com/uploads/media/TPUART2_Datenblatt_20130806.pdf)

## Platforms

* macOS (TCP and Serial)
* Linux (TCP and Serial)
* Cortex M4 (STM32F411)

## Usage

Add `knx_rs` as a dependency in `Cargo.toml`:

```toml
[dependencies]
knx_rs = "*"
```

or for embedded use

```toml
[dependencies]
knx_rs = { version = "*", default-features = false }
```


## Contributors

* mail@uwe-arzt.de Uwe Arzt

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
