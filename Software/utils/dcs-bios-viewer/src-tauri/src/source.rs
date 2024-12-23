use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, UdpSocket};

pub trait Source {
    fn setup(&self) -> Result<(), Error>;
    fn read(&mut self) -> Result<Vec<u8>, Error>;
}

pub struct UdpSource {
    udp_socket: UdpSocket,
    buf: [u8; 1500],
}

impl dcs_bios::source::Source for UdpSource{
    fn setup(&self) -> Result<(),dcs_bios::error::Error> {
        Ok(())
    }

    fn read(&mut self) -> Result<Option<&[u8]>,dcs_bios::error::Error> {
        let a = 0..(self
            .udp_socket
            .recv_from(&mut self.buf)
            .map_err(|e| {
                if e.kind() != ErrorKind::TimedOut {
                    println!("{:?}", e.kind());
                }
                dcs_bios::error::Error::SourceError()
            })?
            .0);
        Ok(Some(&self.buf[a]))
    }
}

impl Source for UdpSource {
    fn setup(&self) -> Result<(), Error> {
        Ok(())
    }

    fn read(&mut self) -> Result<Vec<u8>, Error> {
        let a = 0..(self.udp_socket.recv_from(&mut self.buf)?.0);
        Ok(self.buf[a].to_vec())
    }
}

impl UdpSource {
    pub fn from_addr(bind: &str, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> Result<UdpSource, Error> {
        let udp_socket = UdpSocket::bind(bind)?;
        udp_socket.set_broadcast(true)?;
        udp_socket.set_read_timeout(Some(std::time::Duration::from_secs(1)))?;
        udp_socket.join_multicast_v4(multiaddr, interface)?;
        Ok(UdpSource {
            udp_socket,
            buf: [0; 1500],
        })
    }

    pub fn from_socket(udp_socket: UdpSocket) -> Result<UdpSource, Error> {
        Ok(UdpSource {
            udp_socket,
            buf: [0; 1500],
        })
    }
}