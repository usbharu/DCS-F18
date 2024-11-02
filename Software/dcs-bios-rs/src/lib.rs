#![no_std]

use core::{ops::Range, str};

use error::Error;
use mem::MemoryMap;
use source::Source;

pub mod error;
pub mod mem;
pub mod source;

pub trait DcsBios {
    fn get_integer(&self, address: u16, mask: u16, shift_by: u16) -> Option<u16>;
    fn get_string(&self, address: u16, length: u16) -> Option<&str>;
    fn read(&mut self,listener:&[Listener]) -> Result<(),Error>;
}

pub struct Listener{
    pub address: u16,
    pub func: fn(address:u16)
}

pub struct DcsBiosImpl<S: Source, M: MemoryMap> {
    source: S,
    memory_map: M,
}

impl<S: Source, M: MemoryMap> DcsBiosImpl<S, M> {
    pub fn new(source: S, memory_map: M) -> Self {
        Self { source, memory_map }
    }
}

impl<S: Source, M: MemoryMap> DcsBios for DcsBiosImpl<S, M> {
    fn get_integer(&self, address: u16, mask: u16, shift_by: u16) -> Option<u16> {
        let d = self.memory_map.read(address..(address + 2))?;
        Some((u16::from_le_bytes([d[0], d[1]]) & mask) >> shift_by)
    }

    fn get_string(&self, address: u16, length: u16) -> Option<&str> {
        let d = self.memory_map.read(address..(address + length))?;
        str::from_utf8(d).ok().or(Some("&E&"))
    }

    fn read(&mut self,listener:&[Listener]) -> Result<(),Error> {
        let bytes = self.source.read()?;
        if bytes.is_none() {
            return Ok(());
        };
        let bytes = bytes.unwrap();
        let packet = DcsBiosPacket::new(bytes);
        for ele in packet {
            let address = ele.address;
            let length = ele.length;
            let range = address..(address + length);
            self.memory_map.write(address, ele.data)?;
            for ele in listener {
                if range.contains(&ele.address) {
                    (ele.func)(ele.address);
                }
            }
        }
        Ok(())
    }
}

struct DcsBiosPacket<'a> {
    data: &'a [u8],
    next_offset: usize,
}

impl<'a> DcsBiosPacket<'a> {
    fn new(data: &'a [u8]) -> DcsBiosPacket{
        DcsBiosPacket{
            data,
            next_offset: 0
        }
    }
}

#[derive(Debug)]
struct Receive<'a> {
    address: u16,
    length: u16,
    data: &'a [u8],
}

impl<'a> Iterator for DcsBiosPacket<'a> {
    type Item = Receive<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (receive, offset) = parse_packet_iter(self.data, self.next_offset)?;
        self.next_offset = offset;
        Some(receive)
    }
}

fn parse_packet_iter(data: &[u8], offset: usize) -> Option<(Receive, usize)> {
    let start = if (offset == 0) {
        let sync = data.get(..4)?;
        if sync != [0x55; 4] {
            return None;
        }
        4
    } else {
        offset
    };
    let address = data.get(start..start + 2)?;
    let address = u16::from_le_bytes(match address.try_into() {
        Ok(v) => v,
        Err(_) => return None,
    });
    let len = data.get((start + 2)..(start + 4))?;
    let len: u16 = u16::from_le_bytes(match len.try_into() {
        Ok(v) => v,
        Err(_) => return None,
    });

    let data = data.get((start + 4)..(len as usize + start + 4))?;
    Some((
        Receive {
            address,
            length:len,
            data,
        },
        start + 4 + len as usize,
    ))
}