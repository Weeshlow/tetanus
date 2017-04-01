# Tetanus

A simple cryptolocker tool written in Rust

## Features

#### Lockjaw
Client Module

* AES-256 encryption of files
* Only triggers after receiving the encryption key from c2 server

#### Nail
Command Module

* Creates randomly generated key unique to each client
* Maintains list of victims with their keys
* Communicates the key to the client via a control channel using a custom protocol

## Compiling

1. clone the repo: `git clone https://github.com/rossja/tetanus.git`
2. run `cargo build`

## Usage

1. Run the tetanus binary on the command host with the `--nail` argument
  * Make sure that the command port is reachable from the client (default: 8443
2. Run the tetanus binary on the client host, passing the command host name or address as an argument
