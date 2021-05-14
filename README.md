# Serde XP Protocol

Cross Chain Relay Communication callback protocol

**Initial Reference**

This crate provides a reference implementation of the XP Relay Chain Protocol.
It uses serde & bincode for Serialiazation/Deserialization

# Installation

`Cargo.toml`:
```toml
[dependencies]
serde_xp_protocol = { git = https://github.com/xp-network/serde_xp_protocol }
```

`src.rs`:
```
use serde_xp_protocol::{XpProtocol, Flags, to_bytes, from_bytes}
```

# Testing

`cargo test`

To see debugging info:
`cargo test -- --nocapture`
