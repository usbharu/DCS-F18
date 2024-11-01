// #![no_std]

use crate::packet::Receive;

pub mod packet;
pub mod parser;
//todo イテレータみたいにして10個以上来たときも対応できるようにする
pub fn parse_packet(data: &[u8]) -> Option<[packet::Receive; 10]> {
    let sync = &data[..4];
    if sync != [0x55; 4] {
        return None;
    }

    let mut start = 4;

    let mut r: [Receive; 10] = Default::default();

    let mut count = 0;
    while start < data.len() {
        let address = data.get(start..(start + 2))?;
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
        r[count] = Receive {
            address,
            data_length: len,
            data: data,
        };
        count += 1;

        if count >= 10 {
            break;
        }

        start = start + 4 + len as usize;
    }

    Some(r)
}

pub struct DcsBiosPacket<'a> {
    data: &'a [u8],
    next_offset: usize,
}

impl<'a> DcsBiosPacket<'a> {
    pub  fn new(data: &'a [u8]) -> DcsBiosPacket{
        DcsBiosPacket{
            data,
            next_offset: 0
        }
    }
}

impl<'a> Iterator for DcsBiosPacket<'a> {
    type Item = Receive<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (receive, offset) = parse_packet_iter(self.data, self.next_offset)?;
        self.next_offset = offset;
        Some(receive)
    }
}

pub fn parse_packet_iter(data: &[u8], offset: usize) -> Option<(packet::Receive, usize)> {
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
            data_length: len,
            data,
        },
        start + 4 + len as usize,
    ))
}