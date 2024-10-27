#[derive(Debug)]
pub struct Receive {
    pub address: u16,
    pub data_length: u16,
    pub data: [u8; 2],
}

impl Default for Receive {
    fn default() -> Self {
        Receive{
            address: 0,
            data_length: 0,
            data: [0x00, 0x00],
        }
    }
}
