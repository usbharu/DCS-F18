use std::io::{Error, Read};
use std::net::{Ipv4Addr, TcpStream, UdpSocket};
use std::time::Duration;

pub trait Source {
    fn setup(&self) -> Result<(), Error>;
    fn read(&mut self) -> Result<Vec<u8>, Error>;
}

pub struct UdpSource {
    udp_socket: UdpSocket,
    buf: [u8; 1500],
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
    pub fn from_addr(
        bind: &str,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> Result<UdpSource, Error> {
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

pub struct TcpSource {
    tcp_stream: TcpStream,
    buf: [u8; 1500],
}

impl Source for TcpSource {
    fn setup(&self) -> Result<(), Error> {
        self.tcp_stream
            .set_read_timeout(Some(Duration::from_secs(1)))
    }

    fn read(&mut self) -> Result<Vec<u8>, Error> {
        let range = 0..(self.tcp_stream.read(&mut self.buf)?);
        Ok(self.buf[range].to_vec())
    }
}

impl TcpSource {
    pub fn from_addr(addr: &str) -> Result<TcpSource, Error> {
        Ok(TcpSource {
            tcp_stream: TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(1))?,
            buf: [0; 1500],
        })
    }
}
