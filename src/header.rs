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

use std::fmt;

use byteorder::BigEndian;
use byteorder::ByteOrder;
use num_traits::ToPrimitive;

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
    TunnelRequest = 0x0420,
    TunnelResponse = 0x0421,
    DeviceConfigurationRequest = 0x0310,
    DeviceConfigurationAck = 0x0311,
    RoutingIndication = 0x0530,
}

#[derive(PartialEq)]
pub struct Header {
    _header_length: u8,
    _knxnet_version: u8,
    service_type: ServiceType,
    payload_length: u16,
}

impl Header {
    /// Create an Header
    pub fn new(service_type: ServiceType, payload_length: u16) -> Header {
        Header {
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
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = vec![self._header_length, self._knxnet_version];
        let mut tmp = [0, 2];
        BigEndian::write_u16(&mut tmp, self.service_type.to_u16().unwrap());
        buf.extend(&tmp);
        BigEndian::write_u16(&mut tmp, self.payload_length);
        buf.extend(&tmp);
        buf
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ServiceType: {:?} Length: {}",
            self.service_type, self.payload_length
        )
    }
}
impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} }}", self)
    }
}

// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use crate::header::Header;
    use crate::header::ServiceType;
    use std::mem::size_of;

    #[test]
    fn t_header_length() {
        assert_eq!(size_of::<Header>(), 0x06);
        assert_eq!(Header::length(), 0x06);
    }
    #[test]
    fn t_header_version() {
        assert_eq!(Header::knxnet_version(), 0x10);
    }
    #[test]
    fn t_header_fmt() {
        assert_eq!(
            format!("{}", Header::new(ServiceType::DescriptionRequest, 0x17)),
            "ServiceType: DescriptionRequest Length: 23"
        );
    }
    #[test]
    fn t_header_encode() {
        assert_eq!(
            Header::new(ServiceType::DescriptionRequest, 0x17).encode(),
            &[0x06, 0x10, 0x02, 0x03, 0x00, 0x17]
        );
    }
}
