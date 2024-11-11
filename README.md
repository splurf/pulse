# pulse
Real-time, low-latency audio lounge using Opus codec, designed for smooth, high-quality voice and sound communication. Perfect for interactive gatherings, live discussions, or collaborative spaces, providing clear audio even over variable network conditions.

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
