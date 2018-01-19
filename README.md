# batched-recv-rs

This crate provides batched receiving methods to queues.

## Supported queues

- [`std::sync::mpsc::Receiver`](https://doc.rust-lang.org/stable/std/sync/mpsc/struct.Receiver.html)
- [`crossbeam_channel::Receiver`](https://docs.rs/crossbeam-channel/0.1.2/crossbeam_channel/struct.Receiver.html)
  - only with `crossbeam` feature flag
