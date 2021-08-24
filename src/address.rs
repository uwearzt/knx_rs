// ------------------------------------------------------------------------------
// Copyright 2019-2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::recognize,
    multi::{many0, many1},
    sequence::terminated,
    IResult,
};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash)]
pub struct Address {
    address_type: AddressType,
    main_group: u8,
    mid_group: u8,
    address: u8,
}

#[derive(PartialEq, Eq, Hash)]
pub enum AddressType {
    Individual,
    Group,
}

impl Address {
    /// Create an Address
    pub fn new(address_type: AddressType, main_group: u8, mid_group: u8, address: u8) -> Address {
        Address {
            address_type,
            main_group,
            mid_group,
            address,
        }
    }
    pub fn main(&self) -> u8 {
        self.main_group
    }
    pub fn middle(&self) -> (u8, u8) {
        (self.main_group, self.mid_group)
    }
}

impl FromStr for Address {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = parse_address(s).unwrap();
        let (addrtype, main, middle, address) = ret.1;
        Ok(Address::new(addrtype, main, middle, address))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.address_type {
            AddressType::Group => {
                write!(f, "{}/{}/{}", self.main_group, self.mid_group, self.address)
            }
            AddressType::Individual => {
                write!(f, "{}.{}.{}", self.main_group, self.mid_group, self.address)
            }
        }
    }
}
impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} }}", self)
    }
}

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

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

#[cfg(test)]
mod tests {

    use crate::address::Address;
    use crate::address::AddressType;
    use std::str::FromStr;

    #[test]
    fn t_knx_group_address() {
        assert_eq!(
            Address::from_str("1/2/3").unwrap(),
            Address::new(AddressType::Group, 1, 2, 3)
        );
        assert_eq!(
            Address::from_str("15/15/255").unwrap(),
            Address::new(AddressType::Group, 15, 15, 255)
        );
    }
    #[test]
    fn t_knx_individual_address() {
        assert_eq!(
            Address::from_str("2.3.4").unwrap(),
            Address::new(AddressType::Individual, 2, 3, 4)
        );
        assert_eq!(
            Address::from_str("15.15.255").unwrap(),
            Address::new(AddressType::Individual, 15, 15, 255)
        );
    }
}
