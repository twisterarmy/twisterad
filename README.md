# twisterad

![Build](https://github.com/twisterarmy/twisterad/actions/workflows/build.yml/badge.svg)
[![Dependencies](https://deps.rs/repo/github/twisterarmy/twisterad/status.svg)](https://deps.rs/repo/github/twisterarmy/twisterad)
[![crates.io](https://img.shields.io/crates/v/twisterad.svg)](https://crates.io/crates/twisterad)

Lightweight, in-memory CLI / daemon tool to rotate multiple [twister](https://github.com/twisterarmy/twister-core) ads on a single mining node,
through modified [Bitcoin Core JSON-RPC API](https://github.com/twisterarmy/rust-bitcoincore-rpc) library.

Optimal to run as the systemd unit that waits for a twister connection and then begins updating promotional messages with every new block found.

Check out `config.json` to setup remote / local connection or update default promotional messages for specified users!

## Install

### Stable

``` bash
cargo install twisterad
```
* run `twisterad -c path/to/config.json`

### Repository

* `git clone https://github.com/twisterarmy/twisterad.git`
* `cd twisterad`
* `cargo run -- -c path/to/config.json`

## Options

``` bash
Usage: twisterad [OPTIONS] --config <CONFIG>

Options:
  -c, --config <CONFIG>  Configuration file, required
  -r, --rotate <ROTATE>  Rotate messages time in seconds (`60` by default) [default: 60]
  -h, --help             Print help
  -V, --version          Print version
```