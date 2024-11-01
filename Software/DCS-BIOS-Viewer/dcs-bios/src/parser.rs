use crate::packet::Receive;

type ParserCallback = fn(address: [u16; 1], data: [u16; 1]);

#[derive(PartialEq, Eq, Debug)]
enum State {
    WaitForSync,
    AddressLow,
    AddressHigh,
    CountLow,
    CountHigh,
    DataLow,
    DataHigh,
}

pub struct ProtocolParser {
    state: State,
    sync_byte_count: u8,
    address_buffer: [u8; 2],
    address_uint16: u16,
    count_buffer: [u8; 2],
    count_uint16: u16,
    data_buffer: [u8; 32],
    data_uint16: [u16; 1],
    callback: ParserCallback,
}

impl ProtocolParser {
    pub fn new(callback: ParserCallback) -> Self {
        Self {
            state: State::WaitForSync,
            sync_byte_count: 0,
            address_buffer: [0; 2],
            address_uint16: 0,
            count_buffer: [0; 2],
            count_uint16: 0,
            data_buffer: [0; 32],
            data_uint16: [0; 1],
            callback,
        }
    }

    pub fn process_char<'a>(&'a mut self, c: u8) -> Option<Receive<'a>> {
        let mut r: Option<Receive<'a>> = None;
        match self.state {
            State::WaitForSync => {
                // 処理なし
            }
            State::AddressLow => {
                self.data_buffer = [0;32];
                self.address_buffer[0] = c;
                self.state = State::AddressHigh;
            }
            State::AddressHigh => {
                self.address_buffer[1] = c;
                self.address_uint16 = u16::from_le_bytes(self.address_buffer);
                if self.address_uint16 != 0x5555 {
                    self.state = State::CountLow;
                } else {
                    self.state = State::WaitForSync;
                }
            }
            State::CountLow => {
                self.count_buffer[0] = c;
                self.state = State::CountHigh;
            }
            State::CountHigh => {
                self.count_buffer[1] = c;
                self.count_uint16 = u16::from_le_bytes(self.count_buffer);
                if(self.count_uint16 > 32){
                    println!("count: {}", self.count_uint16);
                }
                
                self.state = State::DataLow;
            }
            State::DataLow => {
                let index = (33usize-self.count_uint16 as usize);
                self.data_buffer[index] = c;
                if self.count_uint16 > 0 {
                    self.count_uint16 -= 1;
                }

                if(self.count_uint16 == 1){
                    self.state = State::DataHigh;
                }

                
            }
            State::DataHigh => {
                // self.data_buffer[1] = c;
                self.data_uint16[0] =
                    u16::from_le_bytes([self.data_buffer[0], self.data_buffer[1]]);
                if self.count_uint16 > 0 {
                    self.count_uint16 -= 1;
                }

                if self.count_uint16 == 0 {
                    // println!("{:?}",self.data_buffer);
                    r = Some(Receive {
                        address: self.address_uint16,
                        data_length: 0,
                        data: &self.data_buffer,
                    });
                    self.state = State::AddressLow;
                } else {
                    self.state = State::DataLow;
                }
                self.address_uint16 = self.address_uint16.wrapping_add(2);
            }
        }

        if c == 0x55 {
            self.sync_byte_count += 1;
        } else {
            self.sync_byte_count = 0;
        }

        if self.sync_byte_count == 4 {
            self.state = State::AddressLow;
            self.sync_byte_count = 0;
        }

        return r;
    }
}
