// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

// #![feature(alloc_error_handler)]
// #![no_main]
// #![no_std]

// use panic_halt as _;

// use cortex_m_rt::entry;
// use stm32f4xx_hal as hal;

// use crate::hal::{pac, prelude::*, serial::config::Config, serial::Serial};

// use core::fmt::Write; // for pretty formatting of the serial output

// use knx_rs::address::Address;
// use knx_rs::address::AddressType;

// #[entry]
// fn main() -> ! {

//     let dp = pac::Peripherals::take().unwrap();
//     let cp = cortex_m::peripheral::Peripherals::take().unwrap();

//     let gpioa = dp.GPIOA.split();

//     let rcc = dp.RCC.constrain();

//     let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();

//     let mut delay = hal::delay::Delay::new(cp.SYST, &clocks);

//     // define RX/TX pins
//     let tx_pin = gpioa.pa2.into_alternate();

//     // configure serial
//     let mut tx = Serial::tx(
//         dp.USART2,
//         tx_pin,
//         Config::default().baudrate(9600.bps()),
//         clocks,
//     )
//     .unwrap();

//     let mut value: u8 = 0;


fn main() {
    let ga01 = Address::new(AddressType::Group, 0x1203);

    loop {
        // print some value every 500 ms, value will overflow after 255
        // writeln!(tx, "value: {:02}\r", value).unwrap();
        writeln!(tx, "value: {:?}\r", ga01.encode()).unwrap();
        // value += 1;
        // delay.delay_ms(500_u32);
    }
}
