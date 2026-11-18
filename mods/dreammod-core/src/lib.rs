//! DreamMod Core: native bootstrap for the interpreted .dmod runtime.
//! This first milestone verifies the native mod ABI before adding runtime features.
use mod_api::Hooks;

pub mod player_snapshot;

#[derive(Default)]
struct DreamModCore;

#[mod_api::horizon_mod]
impl Hooks for DreamModCore {}
