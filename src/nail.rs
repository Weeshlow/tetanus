// nail is the c2 module for the tetanus cryptolocker malware
const DBUG: i32 = 1;

extern crate std;

use std::io;
use std::env;

pub fn init() {
    if DBUG == 1 {
        println!("nail module initialized!");
    }
}
