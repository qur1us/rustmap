# rustmap
Simple [Nmap](https://nmap.org/) wrapper in Rust.

Compilation:

```bash
cd ./rustmap
cargo build --release
ln -s $(pwd)/target/release/rustmap /usr/bin/rustmap
```

Usage:

```bash
_________________________________

rustmap v1.2 by qurius
_________________________________


rustmap <IP ADDRESS> <MACHINE NAME> <OPTIONS>

-h                      This help message.
--speed <number 1-5>    Speed of nmap port scan.
--warp                  Packet minimal rate = 10000 pkt/s.
--verbose               Show ports as nmap finds them in command line.
--no-ping               Tells nmap not to use ping to discover host.
```
