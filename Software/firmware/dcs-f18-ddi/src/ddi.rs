use core::{cell::RefCell, marker::PhantomData};

use crate::Error::{self};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use mcp230xx::{Mcp23017, Mcp230xx};
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

pub enum DDICtl {
    DDI_BRT_CTL,
    DDI_CONT_CTL,
}

pub enum DDISelect {
    DDI_BRT_SELECT,
}

#[derive(Debug)]
pub struct DDI<I2C, E> {
    e: PhantomData<E>,
    ioex_0: Mcp230xx<I2C, Mcp23017>,
    ioex_1: Mcp230xx<I2C, Mcp23017>,
}

impl<'a,I2C2: WriteRead<Error = E> + Write<Error = E>,E> DDI<shared_bus::I2cProxy<'a, shared_bus::NullMutex<I2C2>>, E> {
    fn new(
        bus: &'a BusManager<NullMutex<I2C2>>,
        addresses: Addresses,
    ) -> Result<Self, Error<E>> {
        

        let ioex_0 = match Mcp230xx::new(bus.acquire_i2c(), addresses.ioex_0)
            .map_err(|e| Error::Mcp230xxError(e))
        {
            Ok(o) => o,
            Err(e) => {
                return Err(e);
            }
        };
        let ioex_1: Mcp230xx<shared_bus::I2cProxy<'_, shared_bus::NullMutex<I2C2>>, Mcp23017> = Mcp230xx::new(bus.acquire_i2c(), addresses.ioex_1)
            .map_err(|e| Error::Mcp230xxError(e))?;
        Ok(Self {
            e: PhantomData,
            ioex_0,
            ioex_1
        })
    }

    fn read_push_button(&self, push_button: DDIPushButton) -> Result<bool, Error<E>> {
        todo!()
    }
}
