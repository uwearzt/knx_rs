// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[cfg(feature = "std")]
use strum_macros::{Display, EnumString};

#[cfg(feature = "std")]
#[derive(Display, EnumString, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum DatapointType {
    DPST_1_1,
    DPST_1_5,
    DPST_1_9,

    DPST_3_7,

    DPST_9_1,
    DPST_9_7,
    DPST_9_8,
}
