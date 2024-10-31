#[derive(Debug)]
pub struct Receive<'a> {
    pub address: u16,
    pub data_length: u16,
    pub data: &'a [u8],
}

impl Default for Receive<'_> {
    fn default() -> Self {
        Receive{
            address: 0,
            data_length: 0,
            data: &[0x00, 0x00],
        }
    }
}
