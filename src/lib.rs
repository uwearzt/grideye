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

//! Driver for Panasonic AM88(33)

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

/*fn command(&mut self, command: Command) -> Result<(), Error<E>> {
        let mut cmd_bytes = [0; 2];
        BigEndian::write_u16(&mut cmd_bytes, command.value());
        self.i2c
            .write(self.address as u8, &cmd_bytes)
            .map_err(Error::I2c)
    }

	/// Take a temperature and humidity measurement
    pub fn measure(&mut self, rpt: Repeatability) -> Result<Measurement, Error<E>> {
        self.command(Command::SingleShot(ClockStretch::Disabled, rpt))?;
        self.delay.delay_ms(rpt.max_duration());
        let mut buf = [0; 6];
        self.i2c.read(self.address as u8, &mut buf)
                .map_err(Error::I2c)?;
        let temperature = convert_temperature(BigEndian::read_u16(&buf[0..2]));
        let humidity = convert_humidity(BigEndian::read_u16(&buf[3..5]));
        Ok(Measurement{ temperature, humidity })
    }

	/// Read the status register
    pub fn status(&mut self) -> Result<u16, Error<E>> {
        self.command(Command::Status)?;
        let mut status_bytes = [0; 2];
        self.i2c
            .read(self.address as u8, &mut status_bytes)
            .map_err(Error::I2c)?;
        Ok(BigEndian::read_u16(&status_bytes))
    } */
/*
fn convert_temperature(raw: u16) -> i32 {
    -4500 + (17500 * raw as i32) / 65535
}

fn convert_humidity(raw: u16) -> i32 {
    (10000 * raw as i32) / 65535
}

/// Errors
#[derive(Debug)]
pub enum Error<E> {
    /// Wrong CRC
    Crc,
    /// I2C bus error
    I2c(E),
}

/// I2C address
#[derive(Copy, Clone)]
pub enum Address {
	/// Address pin held high
    High = 0x45,
	/// Address pin held low
    Low = 0x44,
}

enum Command {
    SingleShot(ClockStretch, Repeatability),
    Periodic(Rate, Repeatability),
    FetchData,
    PeriodicWithART,
    Break,
    SoftReset,
    HeaterEnable,
    HeaterDisable,
    Status,
    ClearStatus,
}

enum ClockStretch {
    Enabled,
    Disabled,
}

/// Periodic data acquisition rate
#[allow(non_camel_case_types)]
enum Rate {
	/// 0.5 measurements per second
    R0_5,
	/// 1 measurement per second
    R1,
	/// 2 measurements per second
    R2,
	/// 4 measurements per second
    R4,
	/// 10 measurements per second
    R10,
}

#[derive(Copy, Clone)]
pub enum Repeatability {
    High,
    Medium,
    Low,
}

impl Repeatability {
    /// Maximum measurement duration in milliseconds
    fn max_duration(&self) -> u8 {
        match *self {
            Repeatability::Low => 4,
            Repeatability::Medium => 6,
            Repeatability::High => 15,
        }
    }
}

#[derive(Debug)]
pub struct Measurement {
    pub temperature: i32,
    pub humidity: i32,
}

impl Command {
    fn value(&self) -> u16 {
        use ClockStretch::Enabled as CSEnabled;
        use ClockStretch::Disabled as CSDisabled;
        use Rate::*;
        use Repeatability::*;
        match *self {
            // 4.3 Measurement Commands for Single Shot Data Acquisition Mode
            // Table 8
            Command::SingleShot(CSEnabled,  High)   => 0x2C06,
            Command::SingleShot(CSEnabled,  Medium) => 0x2C0D,
            Command::SingleShot(CSEnabled,  Low)    => 0x2C10,
            Command::SingleShot(CSDisabled, High)   => 0x2400,
            Command::SingleShot(CSDisabled, Medium) => 0x240B,
            Command::SingleShot(CSDisabled, Low)    => 0x2416,

            // 4.5 Measurement Commands for Periodic Data Acquisition Mode
            // Table 9
            Command::Periodic(R0_5, High)   => 0x2032,
            Command::Periodic(R0_5, Medium) => 0x2024,
            Command::Periodic(R0_5, Low)    => 0x202F,
            Command::Periodic(R1,   High)   => 0x2130,
            Command::Periodic(R1,   Medium) => 0x2126,
            Command::Periodic(R1,   Low)    => 0x212D,
            Command::Periodic(R2,   High)   => 0x2236,
            Command::Periodic(R2,   Medium) => 0x2220,
            Command::Periodic(R2,   Low)    => 0x222B,
            Command::Periodic(R4,   High)   => 0x2334,
            Command::Periodic(R4,   Medium) => 0x2322,
            Command::Periodic(R4,   Low)    => 0x2329,
            Command::Periodic(R10,  High)   => 0x2737,
            Command::Periodic(R10,  Medium) => 0x2721,
            Command::Periodic(R10,  Low)    => 0x272A,

            // 4.6 Readout of Measurement Results for Periodic Mode
            // Table 10
            Command::FetchData => 0xE000,

            // 4.7 ART command
            // Table 11
            Command::PeriodicWithART => 0x2B32,

            // 4.8 Break command
            // Table 12
            Command::Break => 0x3093,

            // 4.9 Reset
            // Table 13
            Command::SoftReset => 0x30A2,

            // 4.10 Heater
            // Table 15
            Command::HeaterEnable  => 0x306D,
            Command::HeaterDisable => 0x3066,

            // 4.11 Status register
            // Table 16
            Command::Status => 0xF32D,
            // Table 18
            Command::ClearStatus => 0x3041,
        }
    } 
}  */