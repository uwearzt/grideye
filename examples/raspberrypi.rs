// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// ------------------------------------------------------------------------------

extern crate grideye;
extern crate linux_embedded_hal as hal;

use grideye::{Address, GridEye};
use hal::{Delay, I2cdev};

use std::{thread, time};

fn main() -> Result<(), std::io::Error> {
    println!("GridEye Example");

    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut grideye = GridEye::new(i2c, Delay, Address::Standard);
    grideye.wakeup().unwrap();

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
