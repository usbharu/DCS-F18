#![no_std]

use crate::packet::Receive;

pub mod packet;
//todo イテレータみたいにして10個以上来たときも対応できるようにする
pub fn parse_packet(data: &[u8]) -> Option<[packet::Receive; 10]> {
    let sync = &data[..4];
    if sync != [0x55;4] {
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