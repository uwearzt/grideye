// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f4;

use rt::ExceptionFrame;

extern crate grideye;
use grideye::GridEye;

extern crate embedded_hal;

entry!(main);

fn main() -> ! {
    //let mut stdout = hio::hstdout().unwrap();
    //writeln!(stdout, "Hello, world!").unwrap();

    loop {

    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
