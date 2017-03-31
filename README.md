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

1. run the tetanus binary on the c2 host with the `--nail` argument
2. run the tetanus binary on the victim host passing the ip address of the c2 host as an argument
