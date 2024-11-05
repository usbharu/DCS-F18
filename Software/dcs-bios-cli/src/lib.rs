use std::{
    io::{Error, ErrorKind},
    net::{Ipv4Addr, UdpSocket},
    ops::RangeInclusive,
    path::PathBuf,
};

use dcs_bios::{mem::MemoryMap, source::Source, DcsBios, DcsBiosImpl, Listener};

pub fn main_loop(source: UdpSource) {
    let mut dcs_bios = DcsBiosImpl::new(source, VecMemoryMap::new([0; 65536].to_vec()));

    // let listener = ;

    type Func = fn(u16, &VecMemoryMap) -> ();
    let f: Func = |v: u16, m| {
        let data = DcsBiosImpl::<UdpSource, VecMemoryMap>::get_string(m, 0x0400, 6);
        println!("{}: {:?}", v, data);
    };
    let f2: Func = |v, m| {
        let data = DcsBiosImpl::<UdpSource, VecMemoryMap>::get_integer(m, 1078, 0xfe00, 9);
        println!("{}: {:?}", v, data);
    };
    let f3: Func = |v, m| {
        let data = DcsBiosImpl::<UdpSource, VecMemoryMap>::get_string(m, 0x748a, 6);
        println!("{}: {:?}", v, data);
    };
    let f4: Func = |v, m| {
        let data = DcsBiosImpl::<UdpSource, VecMemoryMap>::get_string(m, 0x7490, 6);
        println!("{}: {:?}", v, data);
    };
    let listeners: &[Listener<VecMemoryMap, Func>] = &[
        Listener {
            address: 1078,
            func: f2,
            _phantom: std::marker::PhantomData,
        },
        Listener {
            _phantom: std::marker::PhantomData,
            address: 0x0400,
            func: f,
        },
        Listener {
            _phantom: std::marker::PhantomData,
            address: 0x748a,
            func: f3,
        },
        Listener {
            _phantom: std::marker::PhantomData,
            address: 0x7490,
            func: f4,
        },
    ];

    loop {
        match dcs_bios.read(listeners) {
            Ok(_) => {}
            Err(e) => {
//                println!("Error: {:?}", e);
            }
        }
    }
}

pub fn setup_source() -> Result<UdpSource, Error> {
    let socket = UdpSocket::bind("0.0.0.0:5010")?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(1)))?;
    socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 50, 10), &Ipv4Addr::new(0, 0, 0, 0))?;
    Ok(UdpSource::new(socket))
}

pub struct UdpSource {
    udp_socket: UdpSocket,
    buf: [u8; 1500],
}

impl UdpSource {
    pub fn new(udp_docket: UdpSocket) -> UdpSource {
        UdpSource {
            udp_socket: udp_docket,
            buf: [0; 1500],
        }
    }
}

impl Source for UdpSource {
    fn setup(&self) -> Result<(), dcs_bios::error::Error> {
        Ok(())
    }

    fn read(&mut self) -> Result<Option<&[u8]>, dcs_bios::error::Error> {
        let a = 0..(self
            .udp_socket
            .recv_from(&mut self.buf)
            .map_err(|e| {
                if e.kind() != ErrorKind::TimedOut {
                    println!("{:?}",e.kind());
                }
                dcs_bios::error::Error::SourceError()
            })?
            .0);
        Ok(Some(&self.buf[a]))
    }
}

#[derive(Debug)]
pub struct VecMemoryMap {
    mem: Vec<u8>,
}

impl VecMemoryMap {
    pub fn new(map: Vec<u8>) -> VecMemoryMap {
        VecMemoryMap { mem: map }
    }
}

impl MemoryMap for VecMemoryMap {
    fn write(
        &mut self,
        address: u16,
        data: &[u8],
    ) -> Result<RangeInclusive<u16>, dcs_bios::error::Error> {
        for (index, ele) in data.iter().enumerate() {
            self.mem[address as usize + index] = *ele;
        }
        Ok(address..=(address + (data.len() as u16 - 1)))
    }

    fn read(&self, range: RangeInclusive<u16>) -> Option<&[u8]> {
        self.mem
            .get((*range.start() as usize)..=(*range.end() as usize))
    }
}

pub fn list_modules(path: PathBuf) -> Vec<String> {
    let mut modules : Vec<String> = path.read_dir()
            .unwrap()
            .filter_map(|v| {
                v.ok().and_then(|p| -> Option<String> {
                    let name = p.file_name();
                    name.into_string().ok()
                })
            })
            .filter(|p| p.ends_with(".json"))
            .map(|p| p.strip_suffix(".json").unwrap().to_string())
            .collect();

    modules.sort();
    modules
}
