mod icmp;
mod ipv4;
mod packets;
mod tcp;

use std::collections::HashMap;

use tcp::{Quad, TcpState};
use tun_tap::{Iface, Mode};

use packets::*;

struct Server {
    connections: HashMap<Quad, TcpState>,
}

impl Server {
    fn new() -> Self {
        Self {
            connections: Default::default(),
        }
    }

    fn serve(&mut self, nic: Iface) {
        loop {
            let mut buf = [0u8; 1504];
            let nbytes = nic.recv(&mut buf[..]).expect("Failed to read from nic");
            if let Some(conn) = process_packet(buf, nbytes) {
                self.connections.entry(conn);
            }
        }
    }
}

fn main() {
    let nic = Iface::new("tun0", Mode::Tun).expect("Failed to create the tun interface");
    let mut server = Server::new();
    server.serve(nic);
}
