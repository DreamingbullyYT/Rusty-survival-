//! DreamMod Core: trusted Horizon native bootstrap and engine bridge.
use mod_api::{Hooks, RegistrationContext, TextureAtlasRegistration};

pub mod atlas_registry;
pub mod engine_bridge;
pub mod engine_coordinates;
pub mod player_snapshot;

#[derive(Default)]
struct DreamModCore;

#[mod_api::horizon_mod]
impl Hooks for DreamModCore {
    fn register_texture_atlases(
        &self,
        _context: &RegistrationContext,
        atlases: &mut Vec<TextureAtlasRegistration>,
    ) {
        atlases.extend(atlas_registry::snapshot());
    }
}
