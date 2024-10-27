#![no_std]

use crate::packet::Receive;

pub mod packet;
pub mod dcs_bios_const;

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
        let address = u16::from_le_bytes(address.try_into()?);
        let len = data.get((start + 2)..(start + 4))?;
        let len: u16 = u16::from_le_bytes(len.try_into()?);
        let x = &data[start..(start + 4 + len as usize)];
        
        let data = data.get((start + 4)..(len as usize + start + 4))?;
        r[count] = Receive {
            address,
            data_length: len,
            data: [data[0], data[1]],
        };
        count += 1;

        if count >= 10 {
            break;
        }

        start = start + 4 + len as usize;
    }

    Some(r)
}