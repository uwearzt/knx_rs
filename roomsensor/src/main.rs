// ------------------------------------------------------------------------------
// Copyright 2021-2022 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use knx_rs::address::{Address, AddressType};
use knx_rs::dpt::{DPT};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    let _ga_temp = Address::from_parts(AddressType::Group, 6, 0, 30);
    let _dpt = DPT::DPT_Value_Temp;
    // let _ga_hum = Address::from_parts(AddressType::Group, 6, 1, 30);
    // let _ga_co2 = Address::from_parts(AddressType::Group, 6, 2, 30);

    loop {
        info!("high");
        led.set_high();
        Timer::after(Duration::from_millis(300)).await;

        info!("low");
        led.set_low();
        Timer::after(Duration::from_millis(300)).await;
    }
}



// use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
// use linux_embedded_hal::Delay;
// use scd4x::Scd4x;

// use linux_embedded_hal::I2cdev;

// use knx_rs::address::{Address, AddressType};

// fn main() {
    // let _ga_temp = Address::from_parts(AddressType::Group, 6, 0, 30);
    // let _ga_hum = Address::from_parts(AddressType::Group, 6, 1, 30);
    // let _ga_co2 = Address::from_parts(AddressType::Group, 6, 2, 30);

    // let dev = I2cdev::new("/dev/i2c-1").unwrap();
    // let mut sensor = Scd4x::new(dev, Delay);

    // sensor.wake_up();
    // sensor.stop_periodic_measurement().unwrap();
    // sensor.reinit().unwrap();

    // let serial = sensor.serial_number().unwrap();

    // println!("serial: {:#04x}", serial);

    // sensor.start_periodic_measurement().unwrap();

    // loop {
    //     Delay.delay_ms(5000u16);
    //     match sensor.measurement() {
    //         Ok(data) => {
    //             println!("CO2: {0}, Temperature: {1:#.2} °C, Humidity: {2:#.2} RH",
    //             data.co2, data.temperature, data.humidity);
    //         }
    //         Err(e) =>  {
    //             println!("error: {:?}", e);
    //         },
    //     }
    // }
// }
