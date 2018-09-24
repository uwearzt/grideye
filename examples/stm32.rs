// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#![no_main]
#![no_std]

extern crate cortex_m;
use cortex_m::asm;

extern crate cortex_m_rt as rt;
use rt::entry;
use rt::exception;

extern crate panic_semihosting;
extern crate stm32f4;
use stm32f4::stm32f405;

use rt::ExceptionFrame;

//extern crate grideye;
//use grideye::GridEye;

extern crate embedded_hal;

#[entry]
fn main() -> ! {

    let peripherals = stm32f405::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    let rcc = &peripherals.RCC;

    rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
    rcc.apb1enr.modify(|_, w| w.tim7en().enabled());

    gpioa.moder.modify(|_, w| w.moder8().output());
    
    loop {
        gpioa.bsrr.write(|w| w.bs8().set());
        asm::delay(10000000);
        gpioa.bsrr.write(|w| w.br8().reset());
        asm::delay(10000000);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
