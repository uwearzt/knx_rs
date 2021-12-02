// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[derive(Primitive, Debug)]
#[repr(u8)]
pub enum IMIMessageCode {
    LBusmonInd = 0x2B,
    LRawDataReq = 0x10,
    // LRawDataCon
    // LRawDataInd
    LDataReq = 0x11,
    LDataCon = 0x2E,
    LDataInd = 0x29,
    LPollDataReq = 0x13,
    LPollDataCon = 0x25,
    // NDataIndividualReq      = 0x13,
    NDataIndividualCon = 0x4E,
    NDataIndividualInd = 0x49,
    NDataGroupReq = 0x22,
    NDataGroupCon = 0x3E,
    NDataGroupInd = 0x3A,
    NDataBroadcastReq = 0x2C,
    NDataBroadcastCon = 0x4F,
    NDataBroadcastInd = 0x4D,
    NPollDataReq = 0x23,
    NPollDataCon = 0x35,
    TConnectReq = 0x43,
    TConnectCon = 0x86,
    TConnectInd = 0x85,
    TDisconnectReq = 0x44,
    TDisconnectCon = 0x88,
    TDisconnectInd = 0x87,
    TDataConnectedReq = 0x41,
    TDataConnectedCon = 0x8E,
    TDataConnectedInd = 0x89,
    TDataGroupReq = 0x32,
    TDataGroupCon = 0x7E,
    TDataGroupInd = 0x7A,
    TDataBroadcastReq = 0x4C,
    TDataBroadcastCon = 0x8F,
    TDataBroadcastInd = 0x8D,
    TDataIndividualReq = 0x4A,
    TDataIndividualCon = 0x9C,
    TDataIndividualInd = 0x94,
    TPollDataReq = 0x33,
    TPollDataCon = 0x75,
    // MConnectReq
    // MConnectCon
    MConnectInd = 0xD5,
    // MDisconnectReq
    // MDisconnectCon
    MDisconnectInd = 0xD7,
    MUserDataConnectedReq = 0x82,
    MUserDataConnectedCon = 0xD1,
    MUserDataConnectedInd = 0xD2,
    ADataGroupReq = 0x72,
    ADataGroupCon = 0xEE,
    ADataGroupInd = 0xEA,
    MUserDataIndividualReq = 0x81,
    MUserDataIndividualCon = 0xDE,
    MUserDataIndividualInd = 0xD9,
    APollDataReq = 0x73,
    APollDataCon = 0xE5,
    MInterfaceObjDataReq = 0x9A,
    MInterfaceObjDataCon = 0xDC,
    MInterfaceObjDataInd = 0xD4,
    UValueReadReq = 0x74,
    UValueReadCon = 0xE4,
    UFlagsReadReq = 0x7C,
    UFlagsReadCon = 0xEC,
    UEventInd = 0xE7,
    UValueWriteReq = 0x71,
    UUserData = 0xD0,
    PcSetValueReq = 0xA6,
    PcGetValueReq = 0xAC,
    PcGetValueCon = 0xAB,
    PeiIdentifyReq = 0xA7,
    PeiIdentifyCon = 0xA8,
    PeiSwitchReq = 0xA9,
    TmTimerInd = 0xC1,
}

/// IMI, entire packet send over KNX TP
pub struct IMI {}

#[derive(PartialEq, Debug)]
pub enum FrameType {
    ExtendedFrame = 0x0,
    StandardFrame = 0x1,
}

#[derive(PartialEq, Debug)]
pub enum RepeatFlag {
    Repeat = 0x0,
    DoNotRepeat = 0x1,
}

#[derive(PartialEq, Debug)]
pub enum SystemBroadcast {
    SystemBroadcast = 0x0,
    Broadcast = 0x1,
}

#[derive(PartialEq, Debug)]
pub enum Priority {
    System = 0x0,
    Normal = 0x1,
    Urgent = 0x2,
    Low = 0x3,
}

#[derive(PartialEq, Debug)]
pub enum AcknowledgeRequest {
    NoACKRequested = 0x0,
    ACKRequested = 0x1,
}

#[derive(PartialEq, Debug)]
pub enum Confirm {
    NoError = 0x0,
    Error = 0x1,
}

#[derive(PartialEq, Debug)]
pub enum ExtendedFrameFormat {
    StandardFrame = 0x0,
}
