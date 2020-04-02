/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/

#![no_std]

use embedded_hal as hal;


/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE> {
    /// Communication error
    Comm(CommE),

}



pub struct NCP5623C<I2C> {
    i2c_port: I2C,
    address: u8,
}


impl<I2C, CommE> NCP5623C<I2C>
    where
        I2C: hal::blocking::i2c::Write<Error = CommE>
        + hal::blocking::i2c::Read<Error = CommE>
        + hal::blocking::i2c::WriteRead<Error = CommE>,
{
    pub fn default(i2c_port: I2C) ->  Result<Self, Error<CommE>> {
        Self::new(i2c_port, DEFAULT_I2C_ADDRESS)
    }

    pub fn new(i2c_port: I2C, address: u8) -> Result<Self, Error<CommE>> {
        let mut inst = Self {
            i2c_port,
            address
        };

        inst.reset()?;
        Ok(inst)
    }

    pub fn reset(&mut self) -> Result<(), Error<CommE>> {
        // Turn off the LED current
        self.set_register_bits(IREG_SHUTDOWN, 0)
    }

    /// You can use BRIGHTNESS_xxx constants here
    pub fn set_brightness(&mut self, current: u8) -> Result<(), Error<CommE>> {
        self.set_register_bits( IREG_LED_CURRENT,  current)
    }

    /// This device has a single 8-bit register and provides "internal register bits" for writing.
    /// See datasheet "Internal Register Selection" and "Table 1. Internal Register Bits Assigment"
    pub fn set_register_bits(&mut self, reg: u8, val: u8)  -> Result<(), Error<CommE>> {
        // ensure that register and value are masked to top 3 and lower 5 bits, respectively
        let write_buf = [(reg & IREG_REG_MASK) | (val & IREG_VAL_MASK)];
        self.i2c_port
            .write(self.address, &write_buf)
            .map_err(Error::Comm)?;
        Ok(())
    }

    /// Set the color and brightness values all at once
    pub fn set_color_brightness(&mut self, bright: u8, red: u8, green: u8, blue: u8)
        -> Result<(), Error<CommE>> {
        // register values are masked to the lower 5 bits
        let write_buf = [
            IREG_LED_CURRENT | (bright & IREG_VAL_MASK),
            IREG_DIM_STEP_RUN,
            IREG_PWM1 | (red & IREG_VAL_MASK),
            IREG_DIM_STEP_RUN,
            IREG_PWM2 | (green & IREG_VAL_MASK),
            IREG_DIM_STEP_RUN,
            IREG_PWM3 | (blue & IREG_VAL_MASK),
        ];

        self.i2c_port
            .write(self.address, &write_buf)
            .map_err(Error::Comm)?;
        Ok(())
    }
}

const DEFAULT_I2C_ADDRESS: u8 = 0x39;
const IREG_VAL_MASK: u8 = 0x1f;
const IREG_REG_MASK: u8 = 0xe0;

/// Constants for brightness
pub const BRIGHTNESS_MAX: u8 = 0x1f;
pub const BRIGHTNESS_HALF: u8 = 0x18;
pub const BRIGHTNESS_LOW: u8 = 0x0f;
pub const BRIGHTNESS_OFF: u8 = 0x00;

/// Internal register bits
pub const IREG_SHUTDOWN: u8 = 0x00;
pub const IREG_LED_CURRENT: u8 = 0x20;
pub const IREG_PWM1: u8 = 0x40;
pub const IREG_PWM2: u8 = 0x60;
pub const IREG_PWM3: u8 = 0x80;
pub const IREG_UPWARD_LEND: u8 = 0xA0;
pub const IREG_DOWNWARD_LEND: u8 = 0xC0;
pub const IREG_DIM_STEP_RUN: u8 = 0xE0;


