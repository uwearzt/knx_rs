#!/bin/bash
set -ex

cargo build
cargo test

cd roomsensor
cargo build