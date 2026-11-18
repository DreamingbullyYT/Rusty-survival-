//! Narrow bridge from the HNMod host API to shared vanilla engine primitives.
//!
//! This module deliberately exposes pure coordinate conversion only. It does not
//! retain Horizon handles, access world state, or claim that a playable engine exists.

use crate::engine_coordinates::{
    BlockPos, ChunkAddress, ChunkPos, SectionAddress, SectionPos,
};

/// Owned result returned across the native/mod boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockLocation {
    pub block: BlockPos,
    pub chunk: ChunkPos,
    pub local_x: u8,
    pub local_z: u8,
    pub section: SectionPos,
    pub local_y: u8,
}

/// Convert an absolute block position using the shared engine implementation.
#[must_use]
pub fn locate_block(x: i32, y: i32, z: i32) -> BlockLocation {
    let block = BlockPos::new(x, y, z);
    let address: ChunkAddress = block.chunk_address();
    let section: SectionAddress = block.section_address();
    BlockLocation {
        block,
        chunk: address.chunk(),
        local_x: address.local_x(),
        local_z: address.local_z(),
        section: section.section(),
        local_y: section.local_y(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_uses_shared_engine_rules_for_negative_coordinates() {
        let location = locate_block(-1, 64, -17);
        assert_eq!(location.block, BlockPos::new(-1, 64, -17));
        assert_eq!(location.chunk, ChunkPos::new(-1, -2));
        assert_eq!(location.local_x, 15);
        assert_eq!(location.local_z, 15);
        assert_eq!(location.section, SectionPos::new(-1, 4, -2));
        assert_eq!(location.local_y, 0);
    }

    #[test]
    fn bridge_preserves_chunk_boundaries() {
        let location = locate_block(16, 0, 32);
        assert_eq!(location.chunk, ChunkPos::new(1, 2));
        assert_eq!(location.local_x, 0);
        assert_eq!(location.local_z, 0);
        assert_eq!(location.section, SectionPos::new(1, 0, 2));
        assert_eq!(location.local_y, 0);
    }
}
