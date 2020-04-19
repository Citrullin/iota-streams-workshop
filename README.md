# iota-streams-workshop

Simple examples to help developers to work with IOTA Streams.

IOTA Streams is a framework for cryptographic protocols called Applications. Streams ships with an existing application, called Channel. The Channel application builds on and extends functionality known from Masked Authenticated Messaging v0 and v1.0.

Read more about streams on [https://blog.iota.org/](https://blog.iota.org/iota-streams-alpha-7e91ee326ac0)

This workshops uses the Rust implementation of [IOTA Streams](https://github.com/iotaledger/streams).

## Setup Workshop

Clone the repository and enter the directory
```bash
git git clone --recurse-submodules https://github.com/Citrullin/iota-streams-workshop
cd iota-streams-workshop
```

Build it with [Cargo](https://doc.rust-lang.org/cargo/)
```bash
cargo build
```

Run code
```bash
./target/debug/iota_stream_example
```