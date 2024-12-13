#![no_std]

pub mod ddi;

pub enum Error<E> {
    Mcp23017Error(mcp23017::Error<E>),
    I2cError(E),
    IllegalInputTypeError,
}