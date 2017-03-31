# Tetanus

A simple cryptolocker malware written in Rust

## Features

* AES-256 encryption of the files
* randomly generated passphrase for encryption
* communicates the passphrase to a control channel via a custom protocol


## Compiling

1. clone the repo
2. run `cargo build`

## Usage

1. run the tetanus binary on the c2 host with the `lockjaw` argument
2. run the tetanus binary on the victim host with the `nail=<c2 ip address>` argument
