#![no_std]

pub mod ddi;

pub enum Error<E> {
    Mcp230xxError(mcp230xx::Error<E>)
}