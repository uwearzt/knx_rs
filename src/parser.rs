// ------------------------------------------------------------------------------
// Copyright 2019-2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use crate::address::Address;
use crate::address::AddressType;
use crate::cemi::CEMIMessageCode;
use crate::cemi::CEMI;
use crate::header::Header;
use crate::header::ServiceType;
use crate::imi::IMI;

use nom::{
    IResult,
    number::complete::{be_u16, be_u8},
};

use num_traits::FromPrimitive;

// ------------------------------------------------------------------------------
fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (i, _header_length) = be_u8(input)?;
    let (i, _protocol_version) = be_u8(i)?;
    let (i, service_type) = be_u16(i)?;
    let (i, payload_length) = be_u16(i)?;

    Ok((
        &i,
        (Header::new(ServiceType::from_u16(service_type).unwrap(), payload_length)),
    ))
}
fn parse_msg_code(input: &[u8]) -> IResult<&[u8], CEMIMessageCode> {
    let (i, msg_code) = be_u8(input)?;
    Ok((&i, (CEMIMessageCode::from_u8(msg_code).unwrap())))
}
fn parse_add_info(input: &[u8]) -> IResult<&[u8], bool> {
    let (i, _add_info_length) = be_u8(input)?;
    Ok((&i, (false)))
}
fn parse_control_field(input: &[u8]) -> IResult<&[u8], bool> {
    let (i, _x) = be_u8(input)?;
    let (i, _y) = be_u8(i)?;
    Ok((&i, (false)))
}

// ------------------------------------------------------------------------------
fn parse_bin_address(input: &[u8]) -> IResult<&[u8], (u8, u8, u8)> {
    let (i, main_middle) = be_u8(input)?;
    let (i, address) = be_u8(i)?;

    let main = main_middle & 0x0f;
    let middle = (main_middle & 0xf0) >> 4;

    Ok((&i, (main, middle, address)))
}
fn parse_phys_address(input: &[u8]) -> IResult<&[u8], Address> {
    let (i, x) = parse_bin_address(input)?;
    Ok((&i, (Address::new(AddressType::Individual, x.0, x.1, x.2))))
}
fn parse_group_address(input: &[u8]) -> IResult<&[u8], Address> {
    let (i, x) = parse_bin_address(input)?;
    Ok((&i, (Address::new(AddressType::Group, x.0, x.1, x.2))))
}

// ------------------------------------------------------------------------------
fn parse_knx_data(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
    let (i, data_len) = be_u8(input)?;
    let (i, _x1) = be_u8(i)?;
    let (i, x2) = be_u8(i)?;
    Ok((&i, (data_len, x2)))
}

// ------------------------------------------------------------------------------
fn cemi(input: &[u8]) -> IResult<&[u8], (Header, CEMIMessageCode, Address, Address, (u8, u8))> {
    let (i, ip_header) = parse_header(input)?;
    let (i, msg_code) = parse_msg_code(i)?;
    let (i, _add_info) = parse_add_info(i)?;
    let (i, _control_field) = parse_control_field(i)?;
    let (i, source_address) = parse_phys_address(i)?;
    let (i, destination_address) = parse_group_address(i)?;
    let (i, data) = parse_knx_data(i)?;

    Ok((
        &i,
        (
            ip_header,
            msg_code,
            source_address,
            destination_address,
            data,
        ),
    ))
}
fn imi(input: &[u8]) -> IResult<&[u8], (Address, Address, (u8, u8))> {
    let (i, _control_byte) = be_u8(input)?;
    let (i, source_address) = parse_phys_address(i)?;
    let (i, destination_address) = parse_group_address(i)?;
    let (i, data) = parse_knx_data(i)?;

    Ok((&i, (source_address, destination_address, data)))
}

// ------------------------------------------------------------------------------
pub fn parse_cemi(bytes: &[u8]) -> CEMI {
    let y = cemi(bytes);
    println!("parse_cemi {:?}", y);
    CEMI::new(Header::new(ServiceType::DescriptionRequest, 0x17))
}
pub fn parse_imi(bytes: &[u8]) -> IMI {
    let y = imi(bytes);
    println!("parse_imi {:?}", y);
    IMI {}
}

// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::parser::parse_cemi;
    use crate::parser::parse_imi;

    #[test]
    fn t_parse_cemi() {
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0x9C, 0xE0, 0x00, 0x00, 0x01, 0x1E,
            0x01, 0x00, 0x81,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0x9C, 0xE0, 0x00, 0x00, 0x01, 0x1E,
            0x01, 0x00, 0x80,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x2C, 0x01, 0x32,
            0x01, 0x00, 0x81,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x2C, 0x01, 0x34,
            0x01, 0x00, 0x89,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x2C, 0x01, 0x34,
            0x01, 0x00, 0x88,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x2C, 0x01, 0x32,
            0x01, 0x00, 0x80,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x2C, 0x09, 0x32,
            0x01, 0x00, 0x80,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x34, 0x03, 0x1E,
            0x01, 0x00, 0x81,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x39, 0x00, 0x1F,
            0x01, 0x00, 0x80,
        ]);
        parse_cemi(&[
            0x06, 0x10, 0x05, 0x30, 0x00, 0x11, 0x29, 0x00, 0xBC, 0xD0, 0x11, 0x42, 0x00, 0x0A,
            0x01, 0x00, 0x80,
        ]);
    }

    #[test]
    fn t_parse_imi() {
        parse_imi(&[0xBC, 0x11, 0x31, 0x01, 0x1E, 0xE1, 0x00, 0x81, 0x1C]);
        parse_imi(&[0xBC, 0x11, 0x31, 0x01, 0x1E, 0xE1, 0x00, 0x80, 0x1D]);
        parse_imi(&[0xBC, 0x11, 0x2C, 0x01, 0x32, 0xE1, 0x00, 0x81, 0x2D]);
        parse_imi(&[0xBC, 0x11, 0x2C, 0x01, 0x34, 0xE1, 0x00, 0x81, 0x2B]);
        parse_imi(&[0xBC, 0x11, 0x2C, 0x01, 0x34, 0xE1, 0x00, 0x80, 0x2A]);
        parse_imi(&[0xBC, 0x11, 0x2C, 0x01, 0x32, 0xE1, 0x00, 0x80, 0x2C]);
        parse_imi(&[0xBC, 0x11, 0x2C, 0x09, 0x33, 0xE1, 0x00, 0x81, 0x24]);
        parse_imi(&[0xBC, 0x11, 0x2C, 0x09, 0x32, 0xE1, 0x00, 0x81, 0x25]);
    }
}
