// ------------------------------------------------------------------------------
// Copyright 2021-2022 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use linux_embedded_hal::Delay;
use scd4x::Scd4x;

use linux_embedded_hal::I2cdev;

use knx_rs::address::{Address, AddressType};

fn main() {
    let _ga_temp = Address::from_parts(AddressType::Group, 6, 0, 30);
    let _ga_hum = Address::from_parts(AddressType::Group, 6, 1, 30);
    let _ga_co2 = Address::from_parts(AddressType::Group, 6, 2, 30);

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Scd4x::new(dev, Delay);

    sensor.wake_up();
    sensor.stop_periodic_measurement().unwrap();
    sensor.reinit().unwrap();

    let serial = sensor.serial_number().unwrap();

    println!("serial: {:#04x}", serial);

    sensor.start_periodic_measurement().unwrap();

    loop {
        Delay.delay_ms(5000u16);
        match sensor.measurement() {
            Ok(data) => {
                println!("CO2: {0}, Temperature: {1:#.2} °C, Humidity: {2:#.2} RH",
                data.co2, data.temperature, data.humidity);
            }
            Err(e) =>  {
                println!("error: {:?}", e);
            },
        }
    }
}


// ------------------------------------------------------------------------------

// #![feature(lang_items)]
// #![no_std]
// #![no_main]

// extern crate libc;
// use libc::c_uint;
// use libc::c_ulong;

// use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
// use linux_embedded_hal::Delay;
// use scd4x::scd4x::Scd4x;

// use linux_embedded_hal::I2cdev;

// use knx_rs::address::{Address, AddressType};

// // used from libc
// extern {
//     pub fn printf(format: *const u8, ...) -> i32;
// }

// #[no_mangle]
// pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
//     // Since we are passing a C string the final null character is mandatory
//
// }
