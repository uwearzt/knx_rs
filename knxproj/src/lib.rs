// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use knx_rs::address::Address;
use knx_rs::dpt::DPT;
use minidom::Element;
use std::fmt;
use std::fs::File;
use std::io::Read;

use std::path::PathBuf;
// use std::str::FromStr;

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct GroupAddress {
    address: knx_rs::address::Address,
    name: String,
    dpt: knx_rs::dpt::DPT,
}
impl fmt::Display for GroupAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \t {} {}", self.address, self.name, self.dpt.dpst())
    }
}
#[derive(Debug, PartialEq)]
pub struct KNXproj {
    pub group_adresses: HashMap<u16, GroupAddress>,
}

pub fn load_knxproj(file: &PathBuf) -> Result<KNXproj, u8> {
    let file = File::open(&file).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    println!("Number of files in archive: {}", archive.len());

    let mut file = match archive.by_name("P-0191/0.xml") {
        Ok(file) => file,
        Err(..) => {
            println!("Can not find file P-0191/0.xml");
            return Err(1);
        }
    };
    println!("{}", file.name());

    let mut knxproj = KNXproj {
        group_adresses: HashMap::new(),
    };
    // let mut map = HashMap::new();

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(i) => println!("read {i} bytes"),
        Err(e) => { println!("Error reading file {e}"); return Err(1) },
    }

    let root: Element = match contents.parse::<Element>() {
        Ok(xml) => xml,
        Err(e) => { println!("Error parsing xml {e}"); return Err(1) },
    };

    let ns = root.ns();
    println!("{}", ns);

    let project = root.get_child("Project", &*ns).unwrap();
    //println!("{:?}", project);

    // <Installations>
    //   <Installation>
    let installations = project.get_child("Installations", &*ns).unwrap();
    let installation = installations.get_child("Installation", &*ns).unwrap();

    //     <GroupAddresses>
    //       <GroupRanges>
    //         <GroupRange>
    //           <GroupRange>
    //             <GroupAddress>

    let group_adresses = installation.get_child("GroupAddresses", &*ns).unwrap();
    // println!("{:?}", group_adresses);

    for group_ranges in group_adresses.children() {
        println!("{}", group_ranges.name());
        for main_groups in group_ranges.children() {
            // println!(
            //     "maingroup -> name: {:?}, start: {:?}, end: {:?}",
            //     main_groups.attr("Name").unwrap(),
            //     main_groups.attr("RangeStart").unwrap(),
            //     main_groups.attr("RangeEnd").unwrap(),
            // );
            for middle_groups in main_groups.children() {
                // println!(
                //     "middlegroup -> name: {:?}, start: {:?}, end: {:?}",
                //     middle_groups.attr("Name").unwrap(),
                //     middle_groups.attr("RangeStart").unwrap(),
                //     middle_groups.attr("RangeEnd").unwrap(),
                // );
                for addresses in middle_groups.children() {
                    // println!(
                    //     "name: {:?}, address: {:?}, dpt: {:?}",
                    //     addresses.attr("Name").unwrap(),
                    //     addresses.attr("Address").unwrap(),
                    //     addresses.attr("DatapointType"),
                    // );

                    let numaddr = addresses.attr("Address").unwrap().parse::<u16>().unwrap();
                    let edpt = match addresses.attr("DatapointType") {
                        Some(dpt) => DPT::from_dpst(&dpt).unwrap(),
                        None => {
                            println!("(None) Datatype for {} using DPT_Bool ", addresses.attr("Name").unwrap());
                            knx_rs::dpt::DPT::DPT_Bool
                        }
                    };

                    let addr = GroupAddress {
                        address: Address::new(knx_rs::address::AddressType::Group, numaddr),
                        name: addresses.attr("Name").unwrap().to_string(),
                        dpt: edpt,
                    };

                    // println!("{:?}", &addr);
                    knxproj.group_adresses.insert(numaddr, addr);
                }
            }
        }
    }

    Ok(knxproj)
}
#[cfg(test)]
mod tests {}
