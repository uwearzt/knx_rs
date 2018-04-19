// ------------------------------------------------------------------------------
// Copyright 2017-2018 by Uwe Arzt mailto:mail@uwe-arzt.de, https://uwe-arzt.de
// under BSD License, see https://uwe-arzt.de/bsd-license
// ------------------------------------------------------------------------------

use header::Header;

#[derive(Primitive, Debug)]
#[repr(u8)]
pub enum CEMIMessageCode {
    // from network layer to data link layer
    LRawReq = 0x10,
    LDataReq = 0x11,
    LPollDataReq = 0x13,

    // from data link layer to network layer
    LPollDataCon = 0x25,
    LDataInd = 0x29,
    LBusmonInd = 0x2B,
    LRawInd = 0x2D,
    LDataCon = 0x2E,
    LRawCon = 0x2F,
}

/// CEMI, entire packet send as multicast over Ethernet
#[derive(PartialEq, Debug)]
pub struct CEMI {
    header: Header,
}

/// Create an Header
impl CEMI {
    pub fn new(header: Header) -> CEMI {
        CEMI { header: header }
    }
}

// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {}
