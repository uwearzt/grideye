// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

//! Driver for Panasonic AMG88(33)

#![no_std]
#![feature(int_to_from_bytes)]

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
        Ok(temperature_u12_to_f32_celsius(temperature, 0.25))
    }

    // ---- Device temperature ---------------------------------------------------------------------
    pub fn get_device_temperature_raw(&mut self) -> Result<u16, Error<E>> {
        self.get_register_as_u16(Register::ThermistorLsb as u8)
    }

    pub fn get_device_temperature_celsius(&mut self) -> Result<f32, Error<E>> {
        let temperature = self.get_device_temperature_raw()?;
        Ok(temperature_u12_to_f32_celsius(temperature, 0.0625))
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
    pub fn enable_interrupt(&mut self) -> Result<(), Error<E>> {
        let mut icr = self.get_register(Register::IntControl as u8)?;
        self.set_register(Register::IntControl, *icr.set_bit(0, true))?;
        Ok(())
    }
    pub fn disable_interrupt(&mut self) -> Result<(), Error<E>> {
        let mut icr = self.get_register(Register::IntControl as u8)?;
        self.set_register(Register::IntControl, *icr.set_bit(0, false))?;
        Ok(())
    }
    pub fn interrupt_enabled(&mut self) -> Result<bool, Error<E>> {
        let icr = self.get_register(Register::IntControl as u8)?;
        Ok(icr.get_bit(1))
    }
    pub fn interrupt_mode_absolut(&mut self) -> Result<(), Error<E>> {
        let mut icr = self.get_register(Register::IntControl as u8)?;
        self.set_register(Register::IntControl, *icr.set_bit(1, true))?;
        Ok(())
    }
    pub fn interrupt_mode_difference(&mut self) -> Result<(), Error<E>> {
        let mut icr = self.get_register(Register::IntControl as u8)?;
        self.set_register(Register::IntControl, *icr.set_bit(1, true))?;
        Ok(())
    }

    // ---- status ---------------------------------------------------------------------------------
    pub fn interrupt_flag_set(&mut self) -> Result<bool, Error<E>> {
        let status = self.get_register(Register::IntControl as u8)?;
        Ok(status.get_bit(1))
    }
    pub fn pixel_temperature_out_ok(&mut self) -> Result<bool, Error<E>> {
        let status = self.get_register(Register::IntControl as u8)?;
        Ok(status.get_bit(2))
    }
    pub fn device_temperature_out_ok(&mut self) -> Result<bool, Error<E>> {
        let status = self.get_register(Register::IntControl as u8)?;
        Ok(status.get_bit(3))
    }
    pub fn clear_interrupt_flag(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::StatusClear, 0x02)?;
        Ok(())
    }
    pub fn clear_pixel_temperatur_overflow(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::StatusClear, 0x04)?;
        Ok(())
    }
    pub fn clear_device_temperature_overflow(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::StatusClear, 0x08)?;
        Ok(())
    }
    pub fn clear_all_overflow(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::StatusClear, 0x0c)?;
        Ok(())
    }
    pub fn clear_all_status(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::StatusClear, 0x0e)?;
        Ok(())
    }

    // ---- pixel interrupt ------------------------------------------------------------------------
    pub fn pixel_interrupt_enabled(&mut self, pixel: u8) -> Result<bool, Error<E>> {
        let intreg = (Register::IntTableInt0 as u8) + (pixel / 8);
        let pos = pixel % 8;

        let inttable = self.get_register(intreg)?;
        Ok(inttable.get_bit(pos as usize))
    }

    // ----  average -------------------------------------------------------------------------------
    pub fn enable_moving_average(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::ReservedAverage, 0x50)?;
        self.set_register(Register::ReservedAverage, 0x45)?;
        self.set_register(Register::ReservedAverage, 0x57)?;
        self.set_register(Register::Average, 0x20)?;
        self.set_register(Register::ReservedAverage, 0x00)?;
        Ok(())
    }

    pub fn disable_moving_average(&mut self) -> Result<(), Error<E>> {
        self.set_register(Register::ReservedAverage, 0x50)?;
        self.set_register(Register::ReservedAverage, 0x45)?;
        self.set_register(Register::ReservedAverage, 0x57)?;
        self.set_register(Register::Average, 0x00)?;
        self.set_register(Register::ReservedAverage, 0x00)?;
        Ok(())
    }

    pub fn moving_average_enabled(&mut self) -> Result<bool, Error<E>> {
        let avg = self.get_register(Register::Average as u8)?;
        Ok(avg.get_bit(5))
    }

    // ---- interrupt value ------------------------------------------------------------------------
    pub fn set_upper_int_value_celsius(&mut self, celsius: f32) -> Result<(), Error<E>> {
        self.set_upper_int_value_raw(temperature_f32_to_u16_celsius(celsius))
    }

    pub fn set_upper_int_value_raw(&mut self, value: u16) -> Result<(), Error<E>> {
        let bytes = value.to_le_bytes();
        self.set_register(Register::IntLevelUpperLsb, bytes[0])?;
        self.set_register(Register::IntLevelUpperMsb, bytes[1])?;
        Ok(())
    }

    pub fn set_lower_int_value_celsius(&mut self, celsius: f32) -> Result<(), Error<E>> {
        self.set_lower_int_value_raw(temperature_f32_to_u16_celsius(celsius))
    }

    pub fn set_lower_int_value_raw(&mut self, value: u16) -> Result<(), Error<E>> {
        let bytes = value.to_le_bytes();
        self.set_register(Register::IntLevelLowerLsb, bytes[0])?;
        self.set_register(Register::IntLevelLowerMsb, bytes[1])?;
        Ok(())
    }

    pub fn set_int_hysteresis_celsius(&mut self, celsius: f32) -> Result<(), Error<E>> {
        self.set_int_hysteresis_raw(temperature_f32_to_u16_celsius(celsius))
    }

    pub fn set_int_hysteresis_raw(&mut self, value: u16) -> Result<(), Error<E>> {
        let bytes = value.to_le_bytes();
        self.set_register(Register::IntLevelHystLsb, bytes[0])?;
        self.set_register(Register::IntLevelHystMsb, bytes[1])?;
        Ok(())
    }

    pub fn upper_int_value_celsius(&mut self) -> Result<f32, Error<E>> {
        let temperature = self.upper_int_value_raw()?;
        Ok(temperature_u12_to_f32_celsius(temperature, 0.25))
    }

    pub fn upper_int_value_raw(&mut self) -> Result<u16, Error<E>> {
        let intval = self.get_register_as_u16(Register::IntLevelUpperLsb as u8)?;
        Ok(intval)
    }

    pub fn lower_int_value_celsius(&mut self) -> Result<f32, Error<E>> {
        let temperature = self.lower_int_value_raw()?;
        Ok(temperature_u12_to_f32_celsius(temperature, 0.25))
    }

    pub fn lower_int_value_raw(&mut self) -> Result<u16, Error<E>> {
        let intval = self.get_register_as_u16(Register::IntLevelLowerLsb as u8)?;
        Ok(intval)
    }
 
    pub fn hysteresis_int_value_celsius(&mut self) -> Result<f32, Error<E>> {
        let temperature = self.hysteresis_int_value_raw()?;
        Ok(temperature_u12_to_f32_celsius(temperature, 0.25))
    }

    pub fn hysteresis_int_value_raw(&mut self) -> Result<u16, Error<E>> {
        let intval = self.get_register_as_u16(Register::IntLevelHystLsb as u8)?;
        Ok(intval)
    }

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
// ---- conversion -----------------------------------------------------------------------------
fn temperature_u12_to_f32_celsius(temperature: u16, factor: f32) -> f32 {
    // check if temperature is negative
    if !temperature.get_bit(11) {
        temperature as f32 * factor
    } else {
        let mut bnot = !temperature;
        let temp = *bnot.set_bits(11..16, 0b00000);
        temp as f32 * -factor
    }
}
fn temperature_f32_to_u16_celsius(mut celsius: f32) -> u16 {
       let mut neg = false;
        if celsius < 0.0 {
            celsius = celsius.abs();
            neg = true;
        }
        let mut temp = celsius as u16;
        if neg {
            temp = *temp.set_bit(11, true);
        }
        return temp
}
