// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
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
