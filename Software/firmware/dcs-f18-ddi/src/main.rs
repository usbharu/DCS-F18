//! This example shows how to communicate asynchronous using i2c with external chips.
//!
//! Example written for the [`MCP23017 16-Bit I2C I/O Expander with Serial Interface`] chip.
//! (https://www.microchip.com/en-us/product/mcp23017)

#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]
extern crate embassy_rp;

use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, interrupt};
use embassy_rp::i2c::InterruptHandler;
use embassy_rp::peripherals::*;

use mcp230xx::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());
    // I2C
    let i2c_contr = p.I2C1;
    let scl = p.PIN_3;
    let sda = p.PIN_2;

    use embassy_rp::i2c::{Async, Config, I2c};
    let i2c: I2c<'_, I2C1, Async> = I2c::new_async(i2c_contr, scl, sda, Irqs, Config::default());

    

    let new_default: Result<Mcp230xx<I2c<'_, I2C1, Async>, Mcp23017>, Error<embassy_rp::i2c::Error>> = Mcp230xx::<I2c<'_, I2C1, Async>, Mcp23017>::new_default(i2c);
}
