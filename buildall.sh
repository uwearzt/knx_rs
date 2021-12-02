#!/bin/bash
# ------------------------------------------------------------------------------
# Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
# SPDX-License-Identifier: Apache-2.0
# ------------------------------------------------------------------------------

set -e
#set -x

cd knx_rs
./test.sh
cd ..

cd knxproj
# cargo test -- --nocapture
cargo test
cd ..

cd logger
cargo build
cargo build --release
cd ..

cd knx_influxdb
cargo build
cargo build --release
cd ..

cd roomsensor
cargo build
cargo build --release
cd ..
