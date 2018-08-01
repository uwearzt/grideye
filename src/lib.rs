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

extern crate bit_field;
extern crate embedded_hal as hal;

use bit_field::BitField;

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

#[allow(dead_code)]
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

#[derive(Copy, Clone)]
pub enum Power {
    Wakeup = 0x00,
    Sleep = 0x10,
    Standby60Seconds = 0x20,
    Standby10Seconds = 0x21,
}

#[derive(Copy, Clone)]
pub enum Framerate {
    Fps10 = 0x00,
    Fps1 = 0x01,
}

#[allow(dead_code)]
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

    // ---- Sensor array ---------------------------------------------------------------------------
    /// Get pixel value for pixel 0-63 as raw value
    pub fn get_pixel_temperature_raw(&mut self, pixel: u8) -> Result<u16, Error<E>> {
        let pixel_low = Register::TemperatureStart as u8 + (2 * pixel);
        self.get_register_as_u16(pixel_low)
    }

    /// Get pixel value for pixel 0-63 as celsius
    pub fn get_pixel_temperature_celsius(&mut self, pixel: u8) -> Result<f32, Error<E>> {
        let temperature = self.get_pixel_temperature_raw(pixel)?;

        // check if temperature is negative
        if temperature.get_bit(11) == false {
            let celsius: f32 = temperature as f32 * 0.25;
            Ok(celsius)
        } else {
            let mut bnot = !temperature;
            let temp = *bnot.set_bits(11..16, 0b00000);
            let celsius: f32 = temp as f32 * -0.25;
            Ok(celsius)
        }
    }

    // ---- Device temperature ---------------------------------------------------------------------
    pub fn get_device_temperature_raw(&mut self) -> Result<u16, Error<E>> {
        self.get_register_as_u16(Register::ThermistorLsb as u8)
    }

    pub fn get_device_temperature_celsius(&mut self) -> Result<f32, Error<E>> {
        let temperature = self.get_device_temperature_raw()?;

        // check if temperature is negative
        if temperature.get_bit(11) == false {
            let celsius: f32 = temperature as f32 * 0.0625;
            Ok(celsius)
        } else {
            let mut bnot = !temperature;
            let temp = *bnot.set_bits(11..16, 0b00000);
            let celsius: f32 = temp as f32 * -0.0625;
            Ok(celsius)
        }
    }

    // ---- framerate ------------------------------------------------------------------------------
    pub fn set_framerate(&mut self, framerate: Framerate) -> Result<(), Error<E>> {
        self.set_register(Register::Framerate, framerate as u8)
    }
    pub fn get_framerate(&mut self) -> Result<(Framerate), Error<E>> {
        let fps = self.get_register(Register::Framerate as u8)?;
        if fps == 0 {
            Ok(Framerate::Fps10)
        } else {
            Ok(Framerate::Fps1)
        }
    }

    // ---- other ----------------------------------------------------------------------------------
    pub fn power(&mut self, power: Power) -> Result<(), Error<E>> {
        self.set_register(Register::PowerControl, power as u8)
    }

    // ---- interrupt ------------------------------------------------------------------------------

    // ---- internal -------------------------------------------------------------------------------
    fn set_register(&mut self, register: Register, value: u8) -> Result<(), Error<E>> {
        let cmd_bytes = [register as u8, value];
        self.i2c
            .write(self.address as u8, &cmd_bytes)
            .map_err(Error::I2c)
    }
    fn get_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let cmd = [register];
        self.i2c
            .write(self.address as u8, &cmd)
            .map_err(Error::I2c)?;
        let mut buffer = [0];
        self.i2c
            .read(self.address as u8, &mut buffer)
            .map_err(Error::I2c)?;
        Ok(buffer[0])
    }
    fn get_register_as_u16(&mut self, register: u8) -> Result<u16, Error<E>> {
        let cmd = [register];
        self.i2c
            .write(self.address as u8, &cmd)
            .map_err(Error::I2c)?;
        let mut buffer = [0, 0];
        self.i2c
            .read(self.address as u8, &mut buffer)
            .map_err(Error::I2c)?;
        Ok(((buffer[1] as u16) << 8) + (buffer[0] as u16))
    }
}
