// ------------------------------------------------------------------------------
// Copyright 2022 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------
#![no_std]

use knx_macro::*;

#[derive(dpt_types)]
#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
enum DPT {
    #[dpt_type(main = 1, sub = 1)]
    DPT_Switch,
    #[dpt_type(main = 1, sub = 2)]
    DPT_Bool,
    #[dpt_type(main = 9, sub = 1)]
    DPT_Value_Temp,
}

#[test]
fn test_macro() {
    let a = DPT::DPT_Switch;
    let b = DPT::DPT_Bool;
    let c = DPT::DPT_Value_Temp;

    assert_eq!(1, a.main());
    assert_eq!(1, b.main());
    assert_eq!(9, c.main());

    assert_eq!(1, a.sub());
    assert_eq!(2, b.sub());
    assert_eq!(1, c.sub());
}
