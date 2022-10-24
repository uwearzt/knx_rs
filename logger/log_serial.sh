#!/bin/bash
# ------------------------------------------------------------------------------
# Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
# SPDX-License-Identifier: Apache-2.0
# ------------------------------------------------------------------------------

#set -e
#set -x

clear; RUST_LOG=debug cargo run --bin logger -- --serial /dev/ttyUSB0 --knxproj /home/uwe/tmp/Haus.knxproj