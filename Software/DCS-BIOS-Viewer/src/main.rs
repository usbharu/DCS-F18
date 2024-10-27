use std::io::ErrorKind;
use DCS_BIOS_Viewer_ROOT::{read_udp, setup_udp};

fn main() {
    let result = setup_udp().unwrap();
    loop {
        let vec = match read_udp(&result) {
            Ok(d) => {
                d
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    continue;
                }
                // println!("Failed to read UDP Data {}", e);
                continue;
            }
        };
        let packet = match dcs_bios::parse_packet(&vec) {
            None => {
                // println!("vec = {:?}", vec);
                // println!("vec len = {}", vec.len());
                // println!("Failed to parse packet");
                continue;
            }
            Some(d) => d
        };

        for r in &packet {
            if r.address == 0x0422 {
                let x = (u16::from_le_bytes(r.data) & 65535) >> 0;
                println!("{}", x);
            } 
        }
        
        // println!("packet = {:?}", packet);
    }
}


// fn main() {
//     let state: &[u8] = &[0x95,0x7e,0x20,0x38,0x36,0x30,0x20,0x34,0x36,0x34,0x5c,0x03,0xd0,0x01,0x45,0x14,0x2d,0x3d,0xec,0x43] ; // stateをu8のスライスとして定義
//     let start_address: usize = 0; // start_addressをusizeとして定義
//     let mask: u16 = 65535 ; // maskをu16として定義
//     let shift: u32 = 0 ; // shiftをu32として定義
// 
//     // stateをu16のスライスに変換して、指定された位置から値を取得
// 
//     
//     
//     a(state, start_address, mask, shift);
// }
// 
// fn a(state: &[u8], start_address: usize, mask: u16, shift: u32) {
//     let value = u16::from_le_bytes([state[start_address], state[start_address + 1]]);
// 
//     // シフトを行う前に、shiftが32未満であることを確認
//     let shifted_value = if shift < 32 {
//         (value & mask) >> shift
//     } else {
//         0 // または適切なデフォルト値
//     };
// 
//     // 最終的にu32型にキャスト
//     let final_value = shifted_value as u32;
// 
//     println!("{}", final_value);
// }