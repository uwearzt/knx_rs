# knx_rs

[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![Actions Status](https://github.com/uwearzt/knx_rs/workflows/push_pullreq/badge.svg)](https://github.com/uwearzt/knx_rs/actions)
[![crates.io](https://meritbadge.herokuapp.com/knx_rs)](https://crates.io/crates/knx_rs)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fuwearzt%2Fknx_rs.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fuwearzt%2Fknx_rs?ref=badge_shield)
[![Gitter](https://badges.gitter.im/knx_rs/Lobby.svg)](https://gitter.im/knx_rs/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fuwearzt%2Fknx_rs.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fuwearzt%2Fknx_rs?ref=badge_shield)

The `knx_rs` crate implements a
[KNX](https://en.wikipedia.org/wiki/KNX_(standard)) Library able to communicate over IP (Multicast) and Serial.

It is used to create some user specific Actors and Sensors on the twisted
wire bus (TP1).

## Hardware

[TP-UART 2 Evaluation Board](http://www.opternus.com/uploads/media/PCBA_UP117-12_datasheet_v5_2012-05-30.pdf)
[TPUART2](http://www.opternus.com/uploads/media/TPUART2_Datenblatt_20130806.pdf)

## Platforms

For developing purposes i use macOS, and at the moment it is tested on macOS only. Tests for a Cortex M4 controller will be added soon.

## Usage

Add `knx_rs` as a dependency in `Cargo.toml`:

```toml
[dependencies]
knx_rs = "0.0.6"
```

### Cross compiling

## Contributors

* mail@uwe-arzt.de Uwe Arzt

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)


[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fuwearzt%2Fknx_rs.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fuwearzt%2Fknx_rs?ref=badge_large)