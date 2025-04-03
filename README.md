# twisterad

![Build](https://github.com/twisterarmy/twisterad/actions/workflows/build.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/twisterad.svg)](https://crates.io/crates/twisterad)

Lightweight, in-memory CLI / daemon tool to rotate multiple [twister](https://github.com/twisterarmy/twister-core) ads on a single mining node,
through modified [Bitcoin Core JSON-RPC API](https://github.com/twisterarmy/rust-twistercore-rpc) library.

It is optimal to run as a systemd unit that waits for a `twisterd` connection and then begins updating promotional messages with each new block found.

Check out `config.json` to setup `twisterd` connection and update default promotions asset!

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

## CLI

``` bash
Usage: twisterad [OPTIONS] --config <CONFIG>

Options:
  -c, --config <CONFIG>          Configuration file, required
  -d, --delay <DELAY>            Rotation queue delay, seconds [default: 60]
  -m, --mode <MODE>              Rotation mode: * `c` - cycle * `s` - stop, disable worker [default: c]
  -p, --processors <PROCESSORS>  Processors limit to mine
  -q, --quantity <QUANTITY>      Iterations quantity before apply rotation `mode`
  -w, --wait <WAIT>              Wait to server reconnect, seconds [default: 900]
  -h, --help                     Print help
  -V, --version                  Print version
```

## System

To run `twisterad` as the `systemd` unit (background process):

* `cd twisterad` - navigate sources directory
* `cargo build --release` - compile optimized binary
* `useradd twisterad` - create new user for `twisterad` process
* `cp target/release/twisterad /usr/bin/twisterad` - copy binary into native system location
* `chmod 0700 /usr/bin/twisterad` - give required permissions
* `chown twisterad:twisterad /usr/bin/twisterad` - allow user/group access
* `mkdir /var/log/twisterad` - create destination for the logs
* `cp config.conf /etc/twisterad.conf` - copy and customize default config

Create new `systemd` configuration file: `nano /etc/systemd/system/twisterad.service`

``` twisterad.service
[Unit]
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=twisterad
Group=twisterad
ExecStart=/usr/bin/twisterad -c /etc/twisterad.conf
StandardOutput=file:/var/log/twisterad/debug.log
StandardError=file:/var/log/twisterad/error.log

[Install]
WantedBy=multi-user.target
```
* to disable debug output, set `null` for `StandardOutput` or `StandardError`

Apply changes:

* `systemctl daemon-reload` - reload unit configuration
* `systemctl enable` - start on system boot
* `systemctl start twisterad` - launch
* `systemctl status twisterad` - check service status

> [!NOTE]
> After launch, `twisterad` listens for the `twisterd` connection to be established,
> and then begins rotation according to the configuration and startup arguments;
>
> When `twisterd` connection is lost, `twisterad` will wait for reconnection
> and continue rotation from the previous memory state. It could be also useful for the desktop
> users, who running their `twisterd` nodes periodically.
>
