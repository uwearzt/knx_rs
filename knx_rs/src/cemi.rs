// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use crate::address::Address;
use crate::address::AddressType;

use crate::imi::AcknowledgeRequest;
use crate::imi::Confirm;
use crate::imi::ExtendedFrameFormat;
use crate::imi::FrameType;
use crate::imi::Priority;
use crate::imi::RepeatFlag;
use crate::imi::SystemBroadcast;

#[derive(PartialEq, Primitive, Debug)]
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

/// CEMI
//#[derive(PartialEq, Debug)]
#[derive(PartialEq)]
pub struct CEMI {
    msg_code: CEMIMessageCode,
    add_length: u8,
    frametype: FrameType,
    repeat: RepeatFlag,
    broadcast: SystemBroadcast,
    prio: Priority,
    ackreq: AcknowledgeRequest,
    confirm: Confirm,
    addresstype: AddressType,
    hopcount: u8,
    extframe: ExtendedFrameFormat,
    src_address: Address,
    pub dest_address: Address,
    pub data_len: u8,
    tpci: u8,
    pub apci: u8,
}

///
impl CEMI {
    pub fn new(
        msg_code: CEMIMessageCode,
        src_address: Address,
        dest_address: Address,
        data_len: u8,
        tpci: u8,
        apci: u8,
    ) -> CEMI {
        CEMI {
            msg_code: msg_code,
            add_length: 0,
            frametype: FrameType::StandardFrame,
            repeat: RepeatFlag::DoNotRepeat,
            broadcast: SystemBroadcast::SystemBroadcast,
            prio: Priority::Low,
            ackreq: AcknowledgeRequest::NoACKRequested,
            confirm: Confirm::NoError,
            addresstype: AddressType::Individual,
            hopcount: 0,
            extframe: ExtendedFrameFormat::StandardFrame,
            src_address: src_address,
            dest_address: dest_address,
            data_len: data_len,
            tpci: tpci,
            apci: apci,
        }
    }
}

// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {}
