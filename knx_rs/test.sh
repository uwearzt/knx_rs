#!/bin/bash
# ------------------------------------------------------------------------------
# Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
# SPDX-License-Identifier: Apache-2.0
# ------------------------------------------------------------------------------

set -e
#set -x

cargo test
cargo test --no-default-features