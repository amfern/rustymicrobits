//! Write a string to the serial port every half second.
//!
//! Note: This example is for the STM32F745/STM32F746

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use nb::block;

use cortex_m_rt::entry;
use stm32f7xx_hal::{
    pac,
    prelude::*,
    serial::{self, Serial},
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216_000_000.Hz()).freeze();

    let gpioa = p.GPIOA.split();

    // https://github.com/iNavFlight/inav/blob/8ac5703a558a4b1367cf2b64b90355019525e61f/src/main/target/KAKUTEF7/target.h#L31C29-L31C32
    // https://github.com/stm32-rs/stm32f7xx-hal/blob/main/examples/blinky.rs
    let mut led  = gpioa.pa2.into_push_pull_output();

    // let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();

    // https://github.com/iNavFlight/inav/blob/8ac5703a558a4b1367cf2b64b90355019525e61f/src/main/target/KAKUTEF7/target.h#L67-L68
    let tx = gpiob.pb10.into_alternate();
    let rx = gpiob.pb11.into_alternate();

    let serial = Serial::new(
        p.USART3,
        (tx, rx),
        &clocks,
        serial::Config {
            // Default to 115_200 bauds
            ..Default::default()
        },
    );
    let (mut tx, mut rx) = serial.split();

    loop {
        led.set_high(); // turn off LED
        let received = block!(rx.read()).unwrap_or('E' as u8);
        block!(tx.write(received)).ok();

        // turn on LED
        for _ in 0..10_000 {
            led.set_low();
        }
    }
}
