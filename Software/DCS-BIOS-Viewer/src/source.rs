use std::io::Error;
use std::net::{Ipv4Addr, UdpSocket};

pub trait Source {
    fn setup(&self) -> Result<(), Error>;
    fn read(&self) -> Result<Vec<u8>, Error>;
}

pub struct UdpSource {
    udp_socket: UdpSocket,
    buf: [u8; 1500],
}

impl Source for UdpSource {
    fn setup(&self) -> Result<(), Error> {
        Ok(())
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        match &self.udp_socket.recv_from(&mut &self.buf) {
            Ok((size, addr)) => {
                Ok(&self.buf[..size].to_vec())
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

impl UdpSource {
    fn from_addr(bind: &str, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> Result<UdpSource, Error> {
        let udp_socket = UdpSocket::bind(bind)?;
        udp_socket.set_broadcast(true)?;
        udp_socket.set_read_timeout(Some(std::time::Duration::from_secs(1)))?;
        udp_socket.join_multicast_v4(multiaddr, interface)?;
        Ok(UdpSource {
            udp_socket,
            buf: [0; 1500],
        })
    }

    fn from_socket(udp_socket: UdpSocket) -> Result<UdpSource, Error> {
        Ok(UdpSource {
            udp_socket,
            buf: [0; 1500],
        })
    }
}