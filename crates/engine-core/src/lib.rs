//! Small, platform-independent primitives for the Rusty Survival engine.
//!
//! This crate intentionally contains no Android, renderer, filesystem, or
//! gameplay-policy dependencies. Minecraft parity is established by
//! version-pinned reference fixtures, not by these utility tests alone.

pub mod coordinates;

pub use coordinates::{BlockPos, ChunkAddress, ChunkPos, CHUNK_WIDTH};
