use core::marker::PhantomData;

use crate::Error::{self};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use shared_bus::{BusManager, NullMutex};

pub struct Addresses {
    ioex_0: u8,
    ioex_1: u8,
    adc: u8,
}

pub enum DDIPushButton {
    DDI_PB_0,
    DDI_PB_1,
    DDI_PB_2,
    DDI_PB_3,
    DDI_PB_4,
    DDI_PB_5,
    DDI_PB_6,
    DDI_PB_7,
    DDI_PB_8,
    DDI_PB_9,
    DDI_PB_10,
    DDI_PB_11,
    DDI_PB_12,
    DDI_PB_13,
    DDI_PB_14,
    DDI_PB_15,
    DDI_PB_16,
    DDI_PB_17,
    DDI_PB_18,
    DDI_PB_19,
}

pub enum DDIBrtSelectState {
    OFF,
    DAY,
    NIGHT,
}

pub enum IoExCh {
    IO_EX_0_A,
    IO_EX_0_B,
    IO_EX_1_A,
    IO_EX_1_B,
}

#[derive(Debug)]
pub struct DDI<I2C: Write + WriteRead, E> {
    e: PhantomData<E>,
    ioex_0: mcp23017::MCP23017<I2C>,
    ioex_1: mcp23017::MCP23017<I2C>,
}

impl<'a, I2C2: WriteRead<Error = E> + Write<Error = E>, E>
    DDI<shared_bus::I2cProxy<'a, shared_bus::NullMutex<I2C2>>, E>
{
    pub fn new(
        bus: &'a BusManager<NullMutex<I2C2>>,
        addresses: Addresses,
    ) -> Result<Self, Error<E>> {
        let ioex_0 = mcp23017::MCP23017::new(bus.acquire_i2c(), addresses.ioex_0)
            .map_err(|e| Error::Mcp23017Error(e))?;

        let ioex_1 = mcp23017::MCP23017::new(bus.acquire_i2c(), addresses.ioex_1)
            .map_err(|e| Error::Mcp23017Error(e))?;

        Ok(Self {
            e: PhantomData,
            ioex_0,
            ioex_1,
        })
    }

    const fn get_push_button_pin(push_button: DDIPushButton) -> (IoExCh, u8) {
        match push_button {
            DDIPushButton::DDI_PB_0 => (IoExCh::IO_EX_0_A, 0),
            DDIPushButton::DDI_PB_1 => (IoExCh::IO_EX_0_A, 1),
            DDIPushButton::DDI_PB_2 => (IoExCh::IO_EX_0_A, 2),
            DDIPushButton::DDI_PB_3 => (IoExCh::IO_EX_0_A, 3),
            DDIPushButton::DDI_PB_4 => (IoExCh::IO_EX_0_A, 4),
            DDIPushButton::DDI_PB_5 => (IoExCh::IO_EX_0_A, 5),
            DDIPushButton::DDI_PB_6 => (IoExCh::IO_EX_0_A, 6),
            DDIPushButton::DDI_PB_7 => (IoExCh::IO_EX_0_A, 7),
            DDIPushButton::DDI_PB_8 => (IoExCh::IO_EX_0_B, 0),
            DDIPushButton::DDI_PB_9 => (IoExCh::IO_EX_0_B, 1),
            DDIPushButton::DDI_PB_10 => (IoExCh::IO_EX_0_B, 2),
            DDIPushButton::DDI_PB_11 => (IoExCh::IO_EX_0_B, 3),
            DDIPushButton::DDI_PB_12 => (IoExCh::IO_EX_0_B, 4),
            DDIPushButton::DDI_PB_13 => (IoExCh::IO_EX_0_B, 5),
            DDIPushButton::DDI_PB_14 => (IoExCh::IO_EX_0_B, 6),
            DDIPushButton::DDI_PB_15 => (IoExCh::IO_EX_0_B, 7),
            DDIPushButton::DDI_PB_16 => (IoExCh::IO_EX_1_A, 0),
            DDIPushButton::DDI_PB_17 => (IoExCh::IO_EX_1_A, 1),
            DDIPushButton::DDI_PB_18 => (IoExCh::IO_EX_1_A, 2),
            DDIPushButton::DDI_PB_19 => (IoExCh::IO_EX_1_A, 3),
        }
    }

    pub fn read_push_button(&mut self, push_button: DDIPushButton) -> Result<bool, Error<E>> {
        let (channel, pin) = Self::get_push_button_pin(push_button);

        let read = self.read_all_push_button(channel)?;

        Ok((read & (1 << pin)) != 0)
    }

    pub fn read_all_push_button(&mut self, channel: IoExCh) -> Result<u8, Error<E>> {
        let (ioex, port) = match channel {
            IoExCh::IO_EX_0_A => (&mut self.ioex_0, mcp23017::Port::GPIOA),
            IoExCh::IO_EX_0_B => (&mut self.ioex_0, mcp23017::Port::GPIOB),
            IoExCh::IO_EX_1_A => (&mut self.ioex_1, mcp23017::Port::GPIOA),
            IoExCh::IO_EX_1_B => Err(Error::IllegalInputTypeError)?,
        };

        if let IoExCh::IO_EX_1_A = channel {
            return Ok((ioex.read_gpio(port).map_err(|e| Error::I2cError(e))?) & 0b11110000);
        }

        ioex.read_gpio(port).map_err(|e| Error::I2cError(e))
    }

    pub fn read_brt_ctl(&mut self) -> Result<u16, Error<E>> {
        todo!()
    }

    pub fn read_cont_ctl(&mut self) -> Result<u16, Error<E>> {
        todo!()
    }

    pub fn read_brt_select(&mut self) -> Result<DDIBrtSelectState, Error<E>> {
        let read = self
            .ioex_1
            .read_gpio(mcp23017::Port::GPIOB)
            .map_err(|e| Error::I2cError(e))?;

        let read = read & 0b00001111;

        Ok(match read {
            0b00001000 => DDIBrtSelectState::OFF,
            0b00000100 => DDIBrtSelectState::NIGHT,
            0b00000010 => DDIBrtSelectState::DAY,
            _ => DDIBrtSelectState::DAY,
        })
    }
}
