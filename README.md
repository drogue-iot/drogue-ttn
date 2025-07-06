# Rust mappings for The Things Network

[![crates.io](https://img.shields.io/crates/v/drogue-ttn.svg)](https://crates.io/crates/drogue-ttn)
[![docs.rs](https://docs.rs/drogue-ttn/badge.svg)](https://docs.rs/drogue-ttn)
[![Matrix](https://img.shields.io/matrix/drogue-iot:matrix.org)](https://matrix.to/#/#drogue-iot:matrix.org)

Rust mappings for APIs of *The Things Network*.

## Parsing Messages on CLI

There is an example application that can be used to test the parsing of TTN
messages:

    cargo run --example parse-v3 -- <path-to-message.json>
