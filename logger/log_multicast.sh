#!/bin/bash
# ------------------------------------------------------------------------------
# Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
# SPDX-License-Identifier: Apache-2.0
# ------------------------------------------------------------------------------

#set -e
#set -x
clear; RUST_LOG=info cargo run --bin logger -- --multicast --knxproj /Users/uwe/tmp/Haus.knxproj
#clear; RUST_LOG=debug cargo run --bin logger -- --multicast --knxproj /Users/uwe/tmp/Haus.knxproj
#clear; RUST_LOG=debug cargo run --bin logger -- --multicast --knxproj /home/uwe/tmp/Haus.knxproj
