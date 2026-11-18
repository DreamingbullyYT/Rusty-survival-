//! DreamMod Core: trusted Horizon native bootstrap and engine bridge.
use mod_api::Hooks;

pub mod engine_bridge;
pub mod player_snapshot;

#[derive(Default)]
struct DreamModCore;

#[mod_api::horizon_mod]
impl Hooks for DreamModCore {}
