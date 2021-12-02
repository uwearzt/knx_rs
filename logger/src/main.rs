// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[macro_use]
extern crate clap;
use clap::{App, Arg};

use knx_rs::ipheader::ServiceType;
use knx_rs::parser::parse_cemi;
use knx_rs::parser::parse_header;
use knx_rs::parser::parse_dpt_9;
use knx_rs::parser::parse_serial;

use knxproj::KNXproj;

use env_logger;
use log::info;
use serial;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;

use serial::prelude::*;
use std::io::prelude::*;

use std::io::ErrorKind;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud19200,
    char_size: serial::Bits8,
    parity: serial::ParityEven,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

// ------------------------------------------------------------------------------
fn main() {
    env_logger::init();

    let parms = App::new("knx_listen")
        .version(crate_version!())
        .about("listen and log KNX messages serial/multicast")
        .author(crate_authors!())
        .arg(
            Arg::with_name("serial")
                .required(true)
                .conflicts_with("multicast")
                .short("s")
                .long("serial")
                .help("use serial port"),
        )
        .arg(
            Arg::with_name("multicast")
                .required(true)
                .conflicts_with("serial")
                .short("m")
                .long("multicast")
                .help("use multicast"),
        )
        .arg(
            Arg::with_name("serialport")
                .required(false)
                .default_value("/dev/cu.usb_to_knx")
                .short("p")
                .long("serialport")
                .help("serial port device"),
        )
        .arg(
            Arg::with_name("multicast_address")
                .required(false)
                .default_value("224.0.23.12:3671")
                .short("a")
                .long("multicast_address")
                .help("multicast address for knx"),
        )
        .arg(
            Arg::with_name("knxproj")
                .required(true)
                .takes_value(true)
                .short("k")
                .long("knxproj")
                .help("KNX project file exported from ETS"),
        )
        .get_matches();

    let mut knxproj: Option<KNXproj> = None;

    if parms.is_present("knxproj") {
        let knxproj_file = parms.value_of("knxproj").unwrap();
        knxproj = Some(knxproj::load_knxproj(knxproj_file).unwrap());
    }
    if parms.is_present("serial") {
        let serial_port = parms.value_of("serialport").unwrap();
        println!("Listening on serial port: {}", serial_port);
        knx_listen_serial(serial_port);
    }
    if parms.is_present("multicast") {
        let multicast_address = parms.value_of("multicast_address").unwrap();
        println!("Listening on multicast address: {}", multicast_address);
        knx_listen_multicast(multicast_address, knxproj);
    }
}

// ------------------------------------------------------------------------------
fn knx_listen_serial(serial_port: &str) {
    let mut port = serial::open(serial_port).expect("couldn't open serial port");
    port.configure(&SETTINGS)
        .expect("couldn't set serial settings");
    // port.set_timeout(Duration::from_secs(10))
    //     .expect("couldn't set timeout");

    info!("start reading bytes");

    let mut buf = [0u8; 32];
    let mut buf_bytes = 0;

    loop {
        let mut readbuf = [0; 24];
        
        match port.read(&mut readbuf) {
            Ok(nr) => {
                info!("serial: {:02x?} -> {}", &readbuf[0..nr], nr);
                buf[buf_bytes..buf_bytes + nr].clone_from_slice(&readbuf[0..nr]);
                buf_bytes += nr;

                if buf_bytes >= 9 {
                    info!("-----------------------------------------------------------");
                    info!("msg: {:02x?} -> {}", &buf[0..buf_bytes], buf_bytes);
                    let (bytes, len) = parse_serial(&buf).unwrap();
                    match len {
                        1 => {
                            buf_bytes = 0;
                        }
                        3 => {
                            if buf_bytes < 11 {
                                println!("not yet complete");
                            }
                            if buf_bytes == 11 {
                                let (_x, value) = parse_dpt_9(bytes).unwrap();
                                println!("value: {}", value);
                                buf_bytes = 0;
                            }
                            if buf_bytes > 11 {
                                let (_x, value) = parse_dpt_9(bytes).unwrap();
                                println!("value: {}", value);
                                // shift left and get correct buflen
                                buf.copy_within(11..buf_bytes, 0);
                                buf_bytes = 11 - buf_bytes;
                            }
                        }
                        _ => {
                            println!("incomplete message, retry");
                        }
                    }
                }
            }
            Err(err) => {
                if err.kind() != ErrorKind::TimedOut {
                    info!(" result : {:?}", err)
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------
fn knx_listen_multicast(multicast_address: &str, knxproj: Option<KNXproj>) {

    let knxproj = knxproj.unwrap();

    let knx_addr: SocketAddrV4 = multicast_address.parse().unwrap();
    let ip_any = Ipv4Addr::new(0, 0, 0, 0);

    let socket = UdpSocket::bind(&knx_addr).expect("couldn't bind to address");
    socket
        .join_multicast_v4(&knx_addr.ip(), &ip_any)
        .expect("couldn't join multicast address");

    loop {
        let mut buf = [0; 128];
        let (nr_bytes, _from) = socket.recv_from(&mut buf).expect("Didn't receive data");

        let (bytes, hdr) = parse_header(&buf[0..nr_bytes]).unwrap();

        // info!("-----------------------------------------------------------");
        // info!("msg: {} -> {}", nr_bytes, hex_to_string(&buf[0..nr_bytes]));
        // info!("IP Header -> {}", hdr);

        match hdr.service_type {
            ServiceType::RoutingIndication => {
                // info!("-----------------------------------------------------------");
                // info!("msg: {} -> {:02x?}", nr_bytes, &buf[0..nr_bytes]);

                let (bytes, cemi) = parse_cemi(&bytes).unwrap();

                // info!("data: {} -> {:02x?}", bytes.len(), &bytes);
                //info!("CEMI -> {:?}", cemi);
                match cemi.data_len {
                    1 => {
                        if cemi.apci == 0x80 {
                            info!("{:?} -> off", knxproj.group_adresses.get(&cemi.dest_address.as_u16()).unwrap());

                        } else {
                            info!("{:?} -> on", knxproj.group_adresses.get(&cemi.dest_address.as_u16()).unwrap());
                        }
                    }
                    3 => {
                        let (_x, value) = parse_dpt_9(bytes).unwrap();
                        info!("{:?} -> {}", knxproj.group_adresses.get(&cemi.dest_address.as_u16()).unwrap(), value);
                    }
                    _ => {
                        info!("no data");
                    }
                }
            }
            _ => {}
        }
    }
}
