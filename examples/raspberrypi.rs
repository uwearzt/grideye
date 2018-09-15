// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

extern crate grideye;
extern crate linux_embedded_hal as hal;

use grideye::{Address, GridEye, Power};
use hal::{Delay, I2cdev};

use std::{thread, time};

fn main() -> Result<(), std::io::Error> {
    println!("GridEye Example");

    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut grideye = GridEye::new(i2c, Delay, Address::Standard);
    grideye.power(Power::Wakeup).unwrap();

    // get the device temperature
    println!(
        "device temperature: {}",
        grideye.get_device_temperature_celsius().unwrap()
    );

    // read the complete image every 5 secs
    loop {
        println!("-------------------------------------------------------------------------");
        for x in 0..8 {
            for y in 0..8 {
                let pixel = (x * 8) + y;
                let temp = grideye.get_pixel_temperature_celsius(pixel).unwrap();
                print!("{}", temp);
                if y < 7 {
                    print!(";");
                }
            }
            println!("");
        }

        let sleeptime = time::Duration::from_secs(5);
        thread::sleep(sleeptime);
    }
}
