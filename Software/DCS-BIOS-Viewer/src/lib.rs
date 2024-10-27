mod source;

use std::net::{Ipv4Addr, UdpSocket};

pub fn setup_udp() -> Result<UdpSocket, std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:5010")?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(1)))?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(&"239.255.50.10".parse::<Ipv4Addr>().unwrap(), &Ipv4Addr::new(0, 0, 0, 0))?;
    Ok(socket)
}


pub fn read_udp(socket: &UdpSocket) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = [0; 1500];


    match socket.recv_from(&mut buf) {
        Ok((size, addr)) => {
            Ok(buf[..size].to_vec())
        }
        Err(e) => {
            Err(e)
        }
    }
}