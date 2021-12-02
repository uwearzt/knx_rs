#!/bin/bash
# ------------------------------------------------------------------------------
# Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
# SPDX-License-Identifier: Apache-2.0
# ------------------------------------------------------------------------------

#set -e
#set -x
RUST_LOG=debug cargo run --bin logger -- --serial --serialport /dev/cu.usbserial-A403INI8