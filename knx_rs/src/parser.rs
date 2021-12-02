// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use crate::address::parse_group_address;
use crate::address::parse_phys_address;
use crate::address::Address;
use crate::cemi::CEMIMessageCode;
use crate::cemi::CEMI;
use crate::ipheader::IPHeader;
use crate::ipheader::ServiceType;
// use crate::imi::IMI;

// use log::info;

use nom::{
    number::complete::{be_u16, be_u8},
    IResult,
};

use num_traits::FromPrimitive;

// ------------------------------------------------------------------------------
fn parse_msg_code(input: &[u8]) -> IResult<&[u8], CEMIMessageCode> {
    let (i, msg_code) = be_u8(input)?;
    Ok((&i, (CEMIMessageCode::from_u8(msg_code).unwrap())))
}
fn parse_add_info(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, add_info_length) = be_u8(input)?;
    // additional infos not yet supported
    assert_eq!(add_info_length, 0);
    Ok((&i, (add_info_length)))
}
fn parse_control_field(input: &[u8]) -> IResult<&[u8], bool> {
    let (i, _x) = be_u8(input)?;
    let (i, _y) = be_u8(i)?;
    Ok((&i, (false)))
}

// ------------------------------------------------------------------------------
fn parse_knx_data(input: &[u8]) -> IResult<&[u8], (u8, u8, u8)> {
    let (i, data_len) = be_u8(input)?;
    let (i, tpci) = be_u8(i)?;
    let (i, apci) = be_u8(i)?;
    Ok((&i, (data_len, tpci, apci)))
}

// ------------------------------------------------------------------------------
pub fn imi(input: &[u8]) -> IResult<&[u8], (Address, Address, (u8, u8, u8))> {
    let (i, _control_byte) = be_u8(input)?;
    let (i, source_address) = parse_phys_address(i)?;
    let (i, destination_address) = parse_group_address(i)?;
    let (i, data) = parse_knx_data(i)?;

    Ok((&i, (source_address, destination_address, data)))
}

// ------------------------------------------------------------------------------
pub fn parse_header(input: &[u8]) -> IResult<&[u8], IPHeader> {
    let (i, _header_length) = be_u8(input)?;
    let (i, _protocol_version) = be_u8(i)?;
    let (i, service_type) = be_u16(i)?;
    let (i, payload_length) = be_u16(i)?;

    Ok((
        &i,
        (IPHeader::new(ServiceType::from_u16(service_type).unwrap(), payload_length)),
    ))
}

pub fn parse_dpt_9(input: &[u8]) -> IResult<&[u8], f32> {
    let (i, dpt_9) = be_u16(input)?;

    // invalid value
    assert_ne!(dpt_9, 0x7fff);

    let exp: u16 = (dpt_9 >> 11) & 0x000f;

    let mut value: i32;
    // negative
    if dpt_9 >= 0x8000 {
        value = dpt_9 as i32 | (-1 & !0x7ff);
    } else {
        value = dpt_9 as i32 & 0x7ff;
    }

    value <<= exp;

    Ok((&i, value as f32 / 100.0))
}

pub fn parse_cemi(input: &[u8]) -> IResult<&[u8], CEMI> {
    let (i, msg_code) = parse_msg_code(input)?;
    let (i, _add_info) = parse_add_info(i)?;
    let (i, _control_field) = parse_control_field(i)?;
    let (i, source_address) = parse_phys_address(i)?;
    let (i, destination_address) = parse_group_address(i)?;
    let (i, (data_len, tpci, apci)) = parse_knx_data(i)?;

    Ok((
        &i,
        (CEMI::new(
            msg_code,
            source_address,
            destination_address,
            data_len,
            tpci,
            apci,
        )),
    ))
}
pub fn parse_serial(input: &[u8]) -> IResult<&[u8], u8> {
    // info!("parse_serial");
    let (i, _x) = be_u8(input)?; // EIB control Field
    let (i, _source_address) = parse_phys_address(i)?;
    let (i, _destination_address) = parse_group_address(i)?;
    let (i, npci) = be_u8(i)?; // NPCI
    let len = npci & 0b_0000_1111;
    let (i, _tpci) = be_u8(i)?; // TPCI
    let (i, apci) = be_u8(i)?; // APCI
    if len == 1 {
        let _data = apci & 0b_0000_0001;
        // println!("data: {}", data);
    }

    // println!("{} {} {}", source_address, destination_address, len);

    Ok((i, len))
}

// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::parser::parse_cemi;
    //use crate::parser::parse_imi;
    use crate::parser::parse_dpt_9;
    use crate::parser::parse_header;

    // use crate::cemi::CEMI;
    use crate::ipheader::IPHeader;
    use crate::ipheader::ServiceType::RoutingIndication;

    #[test]
    fn t_parse_dpt_9() {
        let (_i, value) = parse_dpt_9(&[0x0c, 0x2a]).unwrap();
        assert_eq!(value, 21.32);
        let (_i, value) = parse_dpt_9(&[0x14, 0x6a]).unwrap();
        assert_eq!(value, 45.2);
        let (_i, value) = parse_dpt_9(&[0x2f, 0x0b]).unwrap();
        assert_eq!(value, 576.96);
        let (_i, value) = parse_dpt_9(&[0x84, 0x18]).unwrap();
        assert_eq!(value, -10.0);
    }
    #[test]
    fn t_parse_multicast_msg() {
        t_parse_multicast(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0x9C, 0xE0, 0x00, 0x00, 0x01, 0x1E,
            0x01, 0x00, 0x81,
        ]);
        t_parse_multicast(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0x9C, 0xE0, 0x00, 0x00, 0x01, 0x1E,
            0x01, 0x00, 0x80,
        ]);
        t_parse_multicast(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x13, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x64, 0x30, 0x14,
            0x03, 0x00, 0x80, 0x0C, 0x2a,
        ]);
        t_parse_multicast(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x13, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x64, 0x31, 0x14,
            0x03, 0x00, 0x80, 0x14, 0x53,
        ]);
    }
    fn t_parse_multicast(data: &[u8]) {
        let (cemidata, hdr) = parse_header(data).unwrap();
        assert_eq!(IPHeader::new(RoutingIndication, data.len() as u16), hdr);
        let (knxdata, cemi) = parse_cemi(cemidata).unwrap();
        //assert_eq!(CEMI::new(), cemi);
        // println!("{:?}", hdr);
        // println!("{:?}", cemi);
        match cemi.data_len {
            1 => {
                if cemi.apci == 0x80 {
                    // println!("off");
                } else {
                    // println!("on");
                }
            }
            3 => {
                let (_x, _value) = parse_dpt_9(knxdata).unwrap();
                // println!("value: {:?}", value);
            }
            _ => {
                // println!("no additional data");
            }
        }
    }

    // #[test]
    // fn t_parse_imi() {
    //     parse_imi(&[0xBC, 0x11, 0x31, 0x01, 0x1E, 0xE1, 0x00, 0x81, 0x1C,]);
    //     parse_imi(&[0xBC, 0x11, 0x31, 0x01, 0x1E, 0xE1, 0x00, 0x80, 0x1D,]);
    //     parse_imi(&[0xBC, 0x11, 0x64, 0x31, 0x14, 0xE3, 0x00, 0x80, 0x14, 0x99, 0xFD,]);
    //     parse_imi(&[0xBC, 0x11, 0x64, 0x30, 0x14, 0xE3, 0x00, 0x80, 0x0C, 0x5C, 0x21,]);
    // }
}
