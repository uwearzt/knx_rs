// ------------------------------------------------------------------------------
// Copyright 2019 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use nom::combinator::rest;

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
        let ret = parse(s).unwrap();
        let (main, middle, address) = ret.1;
        println!("############# {} {} {}", main, middle, address);
        Ok(Address::new(
            AddressType::Group,
            u8::from_str(main).unwrap(),
            u8::from_str(middle).unwrap(),
            u8::from_str(address).unwrap(),
        ))
    }
}

named!(parse<&str, (&str, &str, &str)>,
  do_parse!(
    main: take_until!("/") >>
    tag!("/") >>
    middle: take_until!("/") >>
    tag!("/") >>
    address: rest >>
    ((&main, &middle, &address))
  )
);

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

#[cfg(test)]
mod tests {

    use crate::address::Address;
    use crate::address::AddressType;
    use std::str::FromStr;

    #[test]
    fn t_knx_address() {
        assert_eq!(
            Address::from_str("1/2/3").unwrap(),
            Address::new(AddressType::Group, 1, 2, 3)
        );
        assert_eq!(
            Address::from_str("15/15/255").unwrap(),
            Address::new(AddressType::Group, 15, 15, 255)
        );
    }
}
