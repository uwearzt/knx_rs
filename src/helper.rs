// ------------------------------------------------------------------------------
// Copyright 2019 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

pub fn hex_to_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join(" ")
}
