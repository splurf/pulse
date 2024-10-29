# pulse
A client/server model designed for efficient distribution of audio packets.

## Features
- UDP for packet transmission.
- [opus](https://crates.io/crates/opus) codec for encoding/decoding.
- Configurate tick-rate (default: 128 tps).

## Todo
- Client
  - Cross-platform tray-icon
    - Select Input/output device
    - Change volume of other clients
- Server
  - Implement configurations
  - Potential optimizations
