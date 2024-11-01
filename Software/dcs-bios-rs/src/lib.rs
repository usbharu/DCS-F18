#![no_std]

use core::{error::Error, ops::Range};

use heapless::Vec;

struct MemoryMap {
    map: Vec<u8, 65535>,
}

impl MemoryMap {
    fn write(&mut self, address: usize, data: &[u8]) -> Result<(), u8> {
        for (index, ele) in data.iter().enumerate() {
            self.map.insert(address + index, *ele)?;
        }
        Ok(())
    }

    fn read(&self, range: Range<usize>) -> Option<&[u8]> {
        self.map.get(range)
    }
}
