// ------------------------------------------------------------------------------
// Copyright 2019 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use crate::address::Address;
use crate::address::AddressType;
use crate::cemi::CEMIMessageCode;
use crate::cemi::CEMI;
use crate::header::Header;
use crate::header::ServiceType;
use crate::imi::IMI;

use nom::{be_u16, be_u8};

use num_traits::FromPrimitive;

// ------------------------------------------------------------------------------
named!(parse_header<&[u8], Header>,
    do_parse!(
    _header_length: be_u8 >>
    _protocol_version: be_u8 >>
    service_type: be_u16 >>
    payload_length: be_u16 >>

    (Header::new(
        ServiceType::from_u16(service_type).unwrap(),
        payload_length))
    )
);
named!(parse_msg_code<&[u8], CEMIMessageCode >,
    do_parse!(
        msg_code: be_u8 >>
        (CEMIMessageCode::from_u8(msg_code).unwrap())
    )
);
named!(parse_add_info<&[u8], bool >,
    do_parse!(
        _add_info_length: be_u8 >>
        (false)
    )
);
named!(parse_control_field<&[u8], bool >,
    do_parse!(
        _x: be_u8 >>
        _y: be_u8 >>
        (false)
    )
);

// ------------------------------------------------------------------------------
named!(parse_phys_address<&[u8], Address >,
    do_parse!(
        x : parse_address >>
        (Address::new(AddressType::Individual, x.0, x.1, x.2))
    )
);
named!(parse_group_address<&[u8], Address >,
    do_parse!(
        x : parse_address >>
        (Address::new(AddressType::Group, x.0, x.1, x.2))
    )
);
named!(parse_address<&[u8], (u8, u8, u8) >,
    do_parse!(
        group: bits!(tuple!(take_bits!(u8, 4), take_bits!(u8, 4))) >>
        address: be_u8 >>
        (group.0, group.1, address)
    )
);

// ------------------------------------------------------------------------------
named!(parse_data<&[u8], (u8, u8) >,
    do_parse!(
        data_len: be_u8 >>
        x: bits!(tuple!(
            take_bits!(u8, 2),
            take_bits!(u8, 4),
            take_bits!(u8, 4),
            take_bits!(u8, 6))) >>
        ((data_len, x.3))
    )
);

// ------------------------------------------------------------------------------
named!(cemi<&[u8], (Header, CEMIMessageCode, Address, Address, (u8, u8))>,
    do_parse!(
        ip_header: parse_header >>
        msg_code: parse_msg_code >>
        _add_info: parse_add_info >>
        _control_field: parse_control_field >>
        source_address: parse_phys_address >>
        destination_address: parse_group_address >>
        data: parse_data >>
        (ip_header, msg_code, source_address, destination_address, data)
    )
);
named!(imi<&[u8], (Address, Address, (u8, u8))>,
    do_parse!(
        _control_byte: be_u8 >>
        source_address: parse_phys_address >>
        destination_address: parse_group_address >>
        data: parse_data >>
        (source_address, destination_address, data)
    )
);

// ------------------------------------------------------------------------------
pub fn parse_cemi(bytes: &[u8]) -> CEMI {
    //println!("parse_cemi {}", hex_to_string(bytes));
    let y = cemi(bytes);
    println!("y {:?}", y);
    CEMI::new(Header::new(ServiceType::DescriptionRequest, 0x17))
}
pub fn parse_imi(bytes: &[u8]) -> IMI {
    //println!("parse_ft12 {}", hex_to_string(bytes));
    let y = imi(bytes);
    println!("y {:?}", y);
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
