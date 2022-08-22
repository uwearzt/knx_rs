// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use nom::{number::complete::be_u8, IResult};

#[cfg(feature = "std")]
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::recognize,
    multi::{many0, many1},
    sequence::terminated,
};

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
use std::num::ParseIntError;
#[cfg(feature = "std")]
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash)]
pub struct Address {
    address_type: AddressType,
    address: u16,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AddressType {
    Individual,
    Group,
}

impl Address {
    /// Create an Address
    pub fn new(address_type: AddressType, address: u16) -> Address {
        Address {
            address_type,
            address,
        }
    }
    pub fn from_parts(address_type: AddressType, main: u8, middle: u8, address: u8) -> Address {
        Address {
            address_type,
            address: (main as u16) << 11 | (middle as u16) << 8 | address as u16,
        }
    }

    pub fn main(&self) -> u8 {
        ((self.address & 0b_1111_1000_0000_0000) >> 11) as u8
    }
    pub fn middle(&self) -> u8 {
        ((self.address & 0b_0000_0111_0000_0000) >> 8) as u8
    }
    pub fn address(&self) -> u8 {
        (self.address & 0b_0000_0000_1111_1111) as u8
    }

    pub fn as_u16(&self) -> u16 {
        self.address
    }
    pub fn encode(&self) -> [u8; 2] {
        let mut buf = [0u8; 2];
        buf[0] = self.main() << 3 | self.middle();
        buf[1] = self.address();
        buf
    }
}

// ------------------------------------------------------------------------------
fn parse_bin_address(input: &[u8]) -> IResult<&[u8], u16> {
    let (i, main_middle) = be_u8(input)?;
    let (i, address) = be_u8(i)?;

    Ok((&i, (main_middle as u16) << 8 | address as u16))
}
pub fn parse_phys_address(input: &[u8]) -> IResult<&[u8], Address> {
    let (i, addr) = parse_bin_address(input)?;
    Ok((&i, (Address::new(AddressType::Individual, addr))))
}
pub fn parse_group_address(input: &[u8]) -> IResult<&[u8], Address> {
    let (i, addr) = parse_bin_address(input)?;
    Ok((&i, (Address::new(AddressType::Group, addr))))
}

#[cfg(feature = "std")]
impl FromStr for Address {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = parse_address(s).unwrap();
        let (addrtype, main, middle, address) = ret.1;
        Ok(Address::from_parts(addrtype, main, middle, address))
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.address_type {
            AddressType::Group => {
                write!(f, "{}/{}/{}", self.main(), self.middle(), self.address())
            }
            AddressType::Individual => {
                write!(f, "{}.{}.{}", self.main(), self.middle(), self.address())
            }
        }
    }
}
#[cfg(feature = "std")]
impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} }}", self)
    }
}

#[cfg(feature = "std")]
fn parse_address(input: &str) -> IResult<&str, (AddressType, u8, u8, u8)> {
    let (i, main) = decimal(input)?;
    let (i, delim) = alt((tag("/"), tag(".")))(i)?;
    let (i, middle) = decimal(i)?;
    let (i, _) = tag(delim)(i)?;
    let (i, address) = decimal(i)?;

    let main = u8::from_str(main).unwrap();
    let middle = u8::from_str(middle).unwrap();
    let address = u8::from_str(address).unwrap();

    if "/" == delim {
        Ok((i, (AddressType::Group, main, middle, address)))
    } else {
        Ok((i, (AddressType::Individual, main, middle, address)))
    }
}

#[cfg(feature = "std")]
fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

#[cfg(test)]
mod tests {

    use crate::address::Address;
    use crate::address::AddressType;

    #[cfg(feature = "std")]
    use crate::address::parse_group_address;
    #[cfg(feature = "std")]
    use std::str::FromStr;

    #[test]
    fn t_knx_group_address() {
        let ga01 = Address::new(AddressType::Group, 0x3114);
        assert_eq!(ga01.encode(), [0x31, 0x14]);
        assert_eq!(ga01.as_u16(), 12564);

        let ga02 = Address::new(AddressType::Group, 0xffff);
        assert_eq!(ga02.encode(), [0xff, 0xff]);
        assert_eq!(ga02.as_u16(), 65535);
    }
    #[test]
    #[cfg(feature = "std")]
    fn t_knx_group_address_string() {
        let (_, ga01) = parse_group_address(&[0x31, 0x14]).unwrap();
        assert_eq!(ga01, Address::from_parts(AddressType::Group, 6, 1, 20));
        assert_eq!(ga01, Address::from_str("6/1/20").unwrap());
        assert_eq!(format!("{}", ga01), "6/1/20");

        let (_, ga02) = parse_group_address(&[0x7f, 0xff]).unwrap();
        assert_eq!(ga02, Address::from_parts(AddressType::Group, 15, 7, 255));
        assert_eq!(ga02, Address::from_str("15/7/255").unwrap());
        assert_eq!(format!("{}", ga02), "15/7/255");
    }

    #[test]
    fn t_knx_individual_address() {
        let ia01 = Address::new(AddressType::Individual, 0x1203);
        assert_eq!(ia01.encode(), [0x12, 0x03]);
        assert_eq!(ia01.as_u16(), 4611);

        let ia02 = Address::new(AddressType::Individual, 0xffff);
        assert_eq!(ia02.encode(), [0xff, 0xff]);
        assert_eq!(ia02.as_u16(), 65535);
    }
    #[test]
    #[cfg(feature = "std")]
    fn t_knx_individual_address_string() {
        let ia01 = Address::from_str("6.1.20").unwrap();
        assert_eq!(ia01, Address::from_parts(AddressType::Individual, 6, 1, 20));
        assert_eq!(format!("{}", ia01), "6.1.20");

        let ia02 = Address::from_str("15.7.255").unwrap();
        assert_eq!(
            ia02,
            Address::from_parts(AddressType::Individual, 15, 7, 255)
        );
        assert_eq!(format!("{}", ia02), "15.7.255");
    }
}
