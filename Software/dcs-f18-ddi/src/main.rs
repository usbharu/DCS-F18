//! This example shows how to communicate asynchronous using i2c with external chips.
//!
//! Example written for the [`MCP23017 16-Bit I2C I/O Expander with Serial Interface`] chip.
//! (https://www.microchip.com/en-us/product/mcp23017)

#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

use embassy_executor::Spawner;
use embassy_rp;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_rp::i2c::InterruptHandler;
use embassy_rp::peripherals::*;
use mcp230xx::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);
    // I2C
    let i2c_contr = p.I2C1;
    let scl = p.PIN_3;
    let sda = p.PIN_2;
    let hz = 75;

    use embassy_rp::i2c::{self, Config, InterruptHandler};
    let i2c = i2c::I2c::new_async(i2c_contr, scl, sda, Irqs, Config::default());
    Mcp230xx::<
        embassy_rp::i2c::I2c<'_, embassy_rp::peripherals::I2C1, embassy_rp::i2c::Async>,
        Mcp23017,
    >::new_default(i2c);
}
