// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

// use byteorder::BigEndian;
// use byteorder::ByteOrder;
// use num_traits::ToPrimitive;

const HEADER_LENGTH: u8 = 0x06;
const KNXNET_VERSION: u8 = 0x10;

#[derive(PartialEq, Primitive, Debug)]
#[repr(u16)]
pub enum ServiceType {
    SearchRequest = 0x0201,
    SearchResponse = 0x0202,
    DescriptionRequest = 0x0203,
    DescriptionResponse = 0x0204,
    ConnectionRequest = 0x0205,
    ConnectionResponse = 0x0206,
    ConnectionstateRequest = 0x0207,
    ConnectionstateResponse = 0x0208,
    DisconnectRequest = 0x0209,
    DisconnectResponse = 0x020A,
    Unknown01 = 0x020B,
    TunnelRequest = 0x0420,
    TunnelResponse = 0x0421,
    DeviceConfigurationRequest = 0x0310,
    DeviceConfigurationAck = 0x0311,
    RoutingIndication = 0x0530,
    RoutingLostMessage = 0x0531,
    RoutingBusy = 0x0532,
}

#[derive(PartialEq, Debug)]
pub struct IPHeader {
    _header_length: u8,
    _knxnet_version: u8,
    pub service_type: ServiceType,
    pub payload_length: u16,
}

impl IPHeader {
    /// Create an Header
    pub fn new(service_type: ServiceType, payload_length: u16) -> IPHeader {
        IPHeader {
            _header_length: HEADER_LENGTH,
            _knxnet_version: KNXNET_VERSION,
            service_type: service_type,
            payload_length: payload_length,
        }
    }

    /// Return the byte-size of an `Header`. Always 0x06.
    pub fn length() -> u8 {
        HEADER_LENGTH
    }

    /// Return the `version` field of a header. Always 0x10 (1.0).
    pub fn knxnet_version() -> u8 {
        KNXNET_VERSION
    }
    pub fn encode(&self) -> [u8; HEADER_LENGTH as usize] {
        let mut buf = [0u8; HEADER_LENGTH as usize];
        buf[0] = HEADER_LENGTH;
        buf[1] = KNXNET_VERSION;
        // let mut tmp = [0, 2];
        // BigEndian::write_u16(&mut tmp, self.service_type.to_u16().unwrap());
        // buf.extend(&tmp);
        // BigEndian::write_u16(&mut tmp, self.payload_length);
        // buf.extend(&tmp);
        buf
    }
}

// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use crate::ipheader::IPHeader;
    //use crate::header::ServiceType;

    #[test]
    fn t_header_length() {
        assert_eq!(IPHeader::length(), 0x06);
    }
    #[test]
    fn t_header_version() {
        assert_eq!(IPHeader::knxnet_version(), 0x10);
    }
    /* #[test]
    fn t_header_encode() {
        assert_eq!(
            Header::new(ServiceType::DescriptionRequest, 0x17).encode(),
            &[0x06, 0x10, 0x02, 0x03, 0x00, 0x17]
        );
    } */
}
