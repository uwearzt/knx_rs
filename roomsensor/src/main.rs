// ------------------------------------------------------------------------------
// Copyright 2021-2023 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use knx_rs::address::{Address, AddressType};
use knx_rs::dpt::DPT;

use defmt::*;
use embassy_executor::Spawner;
// use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::dma::NoDma;
use embassy_stm32::time::Hertz;
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::i2c::I2c;
use embassy_stm32::interrupt;
// use embassy_embedded_hal::adapter::BlockingAsync;

//use embassy_stm32::usart::Config;
use embassy_stm32::usart::Config as UsartConf;
use embassy_stm32::usart::Uart;

use scd4x::Scd4x;

#[embassy_executor::task]
async fn test() {
    loop {
        info!("hello from test");
        Timer::after(Duration::from_millis(5000)).await;
    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    //let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    let _ga_temp = Address::from_parts(AddressType::Group, 6, 0, 30);
    let _dpt = DPT::DPT_Value_Temp;

    // let _ga_hum = Address::from_parts(AddressType::Group, 6, 1, 30);
    // let _ga_co2 = Address::from_parts(AddressType::Group, 6, 2, 30);

    let irq = interrupt::take!(I2C1_EV);

    let i2c = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        irq,
        NoDma,
        NoDma,
        Hertz(100_000),
        Default::default(),
    );

    // let mut i2c = BlockingAsync::new(i2c);

    let mut sensor = Scd4x::new(i2c, Delay);
    Timer::after(Duration::from_millis(1000)).await;

    match sensor.serial_number() {
        Ok(serial) => {
            info!("serial: {:#04x}", serial);
        }
        Err(_e) => {
            error!("could not read serial from SCD40");
        }
    }

    let config = UsartConf::default();
    let irq = interrupt::take!(USART2);
    let mut usart = Uart::new(p.USART2, p.PA3, p.PA2, irq, NoDma, NoDma, config);

    unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
    info!("wrote Hello, starting echo");

    unwrap!(spawner.spawn(test()));

    sensor.wake_up();

    loop {
        Timer::after(Duration::from_millis(5000)).await;

        match sensor.measurement() {
            // match sensor.sensor_output() {
            Ok(data) => {
                info!(
                    "CO2: {}, Temperature: {} C, Humidity: {}",
                    data.co2, data.temperature, data.humidity
                );
            }
            Err(_e) => {
                info!("error");
            }
        }
    }
}
