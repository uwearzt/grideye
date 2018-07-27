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

//! Driver for Panasonic AMG88(33)

#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::delay::DelayMs;
use hal::blocking::i2c::{Read, Write, WriteRead};

/// Errors
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),
}
/// I²C address
#[derive(Copy, Clone)]
pub enum Address {
    Standard = 0x69,
    Alternate = 0x68,
}

#[derive(Copy, Clone)]
enum Register {
    PowerControl = 0x00,
    Reset = 0x01,
    Framerate = 0x02,
    IntControl = 0x03,
    Status = 0x04,
    StatusClear = 0x05,
    Average = 0x07,
    IntLevelUpperLsb = 0x08,
    IntLevelUpperMsb = 0x09,
    IntLevelLowerLsb = 0x0A,
    IntLevelLowerMsb = 0x0B,
    IntLevelHystLsb = 0x0C,
    IntLevelHystMsb = 0x0D,
    ThermistorLsb = 0x0E,
    ThermistorMsb = 0x0F,
    IntTableInt0 = 0x10,
    ReservedAverage = 0x1F,
    TemperatureStart = 0x80,
}

pub struct GridEye<I2C, D> {
    i2c: I2C,
    delay: D,
    address: Address,
}

impl<I2C, D, E> GridEye<I2C, D>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    D: DelayMs<u8>,
{
    /// Creates a new driver
    pub fn new(i2c: I2C, delay: D, address: Address) -> Self {
        GridEye {
            i2c,
            delay,
            address,
        }
    }

    /// Get pixel value for pixel 0-63 as raw value
    pub fn get_pixel_temperature_raw(&mut self, pixel: u8) -> Result<u16, Error<E>> {
        let pixel_low = Register::TemperatureStart as u8 + (2 * pixel);
        self.get_register_as_u16(pixel_low)
    }

    /// Get pixel value for pixel 0-63 as raw value
    pub fn get_pixel_temperature_celsius(&mut self, pixel: u8) -> Result<f32, Error<E>> {
        let temperature = self.get_pixel_temperature_raw(pixel)?;
        // temperature is reported as 12-bit twos complement
        // check if temperature is negative
  /*if(temperature & (1 << 11))
  {
    // if temperature is negative, mask out the sign byte and 
    // make the float negative
    temperature &= ~(1 << 11);
    temperature = temperature * -1;
  }*/
        let celsius: f32 = temperature as f32 * 0.25;
        Ok(celsius)
    }

    /// power control
    pub fn wakeup(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::PowerControl, 0x00)
    }

    pub fn sleep(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::PowerControl, 0x10)
    }

    pub fn standby60seconds(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::PowerControl, 0x20)
    }

    pub fn standby10seconds(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::PowerControl, 0x21)
    }

    fn set_register(&mut self, register: Register, value: u8) -> Result<(), Error<E>> {
        let cmd_bytes = [register as u8, value];
        self.i2c
            .write(self.address as u8, &cmd_bytes)
            .map_err(Error::I2c)
    }
    fn get_register_as_u16(&mut self, register: u8) -> Result<u16, Error<E>> {
        let cmd = [register];
        self.i2c.write(self.address as u8, &cmd).map_err(Error::I2c)?;
        let mut buffer = [0, 0];
        self.i2c.read(self.address as u8, &mut buffer).map_err(Error::I2c)?;
        Ok(((buffer[1] as u16) << 8) + (buffer[0] as u16))
    }
}