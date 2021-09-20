use std::net::Ipv4Addr;

use etherparse::Ipv4HeaderSlice;

pub(crate) fn ipv4_headers(p: Ipv4HeaderSlice) -> (Ipv4Addr, Ipv4Addr, u16, u8) {
    (
        p.source_addr(),
        p.destination_addr(),
        p.payload_len(),
        p.ttl(),
    )
}
