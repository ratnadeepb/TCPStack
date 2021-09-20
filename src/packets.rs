use crate::{
    icmp::icmp_handler,
    tcp::{tcp_handler, Quad},
};
use etherparse::Ipv4HeaderSlice;

pub(crate) fn process_packet(buf: [u8; 1504], nbytes: usize) -> Option<Quad> {
    let flags = u16::from_be_bytes([buf[0], buf[1]]);
    let proto = u16::from_be_bytes([buf[2], buf[3]]);

    println!("Ethernet flags: {}", flags);
    if proto != 0x800 {
        return None;
    }

    match Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
        Ok(p) => match p.protocol() {
            0x1 => {
                icmp_handler(p);
                None
            }
            0x06 => {
                let l = p.slice().len();
                tcp_handler(p, &buf[4 + l..nbytes])
            }
            _ => {
                println!("Neither an ICMP or TCP packet");
                None
            }
        },
        Err(e) => {
            eprintln!("Could not get IPv4 Header: {}", e);
            None
        }
    }
}
