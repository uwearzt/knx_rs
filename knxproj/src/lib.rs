// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use knx_rs::address::Address;
use knx_rs::dpt::DatapointType;
use minidom::Element;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use std::collections::HashMap;


#[derive(PartialEq, Debug)]
pub struct GroupAddress {
    address: knx_rs::address::Address,
    name: String,
    dpt: knx_rs::dpt::DatapointType,
}
#[derive(Debug, PartialEq)]
pub struct KNXproj {
    pub group_adresses: HashMap<u16, GroupAddress>,
}


pub fn load_knxproj(file: &str) -> Result<KNXproj, u8> {
    let fname = std::path::Path::new(file);
    let file = File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    println!("Number of files in archive: {}", archive.len());

    let mut file = match archive.by_name("P-0191/0.xml") {
        Ok(file) => file,
        Err(..) => {
            println!("File P-0191/0.xml");
            return Err(1);
        }
    };
    println!("{}", file.name());

    let mut knxproj = KNXproj { group_adresses: HashMap::new(), };
    // let mut map = HashMap::new();


    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);

    let root: Element = contents.parse().unwrap();
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
                        Some(dpt) => DatapointType::from_str(&dpt.replace("-", "_")).unwrap(),
                        None => {
                            println!("(None) Datatype for {} ", addresses.attr("Name").unwrap());
                            knx_rs::dpt::DatapointType::DPST_1_1
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
mod tests {
    use crate::load_knxproj;

    #[test]
    fn load_test() {
        load_knxproj("/Users/uwe/tmp/Haus.knxproj").unwrap();
    }
}
