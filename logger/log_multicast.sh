#!/bin/bash
# ------------------------------------------------------------------------------
# Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
# SPDX-License-Identifier: Apache-2.0
# ------------------------------------------------------------------------------

#set -e
#set -x

clear; RUST_LOG=info cargo run --bin logger -- --multicast 224.0.23.12:3671 --knxproj /Users/uwe/tmp/Haus.knxproj
