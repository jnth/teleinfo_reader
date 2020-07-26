# Teleinfo Reader

Rust CLI program that read serial data from teleinformation flux.
Only work for the TEMPO subscription.

Send data into PostgreSQL database.


## Installation

### Cross compilation for the Raspberry Pi Zero

Install `cross`, the "Zero setup" cross compilation and "cross testing" of Rust crates:

    cargo install cross

Build the binary for the Raspberry Pi Zero:

    cross build --target armv7-unknown-linux-musleabihf --release

Note: `armv7-unknown-linux-musleabihf` is preferred because of the `libudev` dependency.