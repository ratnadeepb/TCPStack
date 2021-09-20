use etherparse::Ipv4HeaderSlice;

use crate::ipv4::*;

pub(crate) fn icmp_handler(p: Ipv4HeaderSlice) {
    let (src, dst, _, ttl) = ipv4_headers(p);
    println!("ICMP packet from {} to {} with ttl = {}", src, dst, ttl);
}
