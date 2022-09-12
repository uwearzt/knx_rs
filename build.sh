#!/bin/bash
# ------------------------------------------------------------------------------
# Copyright 2022 Uwe Arzt, mail@uwe-arzt.de
# SPDX-License-Identifier: Apache-2.0
# ------------------------------------------------------------------------------
set -ex

cargo build
cargo test

cd roomsensor
cargo build