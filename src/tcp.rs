use std::net::Ipv4Addr;

use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};

use crate::ipv4::ipv4_headers;

pub(crate) struct TcpState {}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

pub(crate) fn tcp_handler(p: Ipv4HeaderSlice, buf: &[u8]) -> Option<Quad> {
    let (src, dst, plen, ttl) = ipv4_headers(p);
    println!(
        "TCP packet from {} to {} with payload size = {} bytes with ttl = {}",
        src, dst, plen, ttl
    );
    match TcpHeaderSlice::from_slice(&buf) {
        Ok(p) => {
            let seq_num = p.sequence_number();
            let ack_num = p.acknowledgment_number();
            let src_port = p.source_port();
            let dst_port = p.destination_port();
            let data_offset = p.data_offset();
            let fin = p.fin(); // reading the fin flag
            let syn = p.syn();
            let rst = p.rst();
            let ack = p.ack();
            let window_sz = p.window_size();
            println!(
                "Connection Identifier: ({}:{}->{}:{})",
                src, src_port, dst, dst_port
            );
            print!("sequence number: {}", seq_num);
            print!(" acknowledgement number: {}", ack_num);
            print!(" data offset: {}", data_offset);
            print!(" window size: {}", window_sz);
            print!(" syn?: {}", syn);
            print!(" ack?: {}", ack);
            print!(" reset?: {}", rst);
            println!(" terminate?: {}", fin);
            Some(Quad {
                src: (src, src_port),
                dst: (dst, dst_port),
            })
        }
        Err(e) => {
            eprintln!("Could not get TCP Header: {}", e);
            None
        }
    }
}
