//! Block-to-chunk coordinate mapping.
//!
//! Horizontal chunks are 16 blocks wide. Euclidean division is required so
//! negative world coordinates map to the same 0..16 local range as positives.
//! Vertical dimension/section bounds are deliberately not defined here.

/// Number of blocks along each horizontal chunk edge.
pub const CHUNK_WIDTH: i32 = 16;

/// Integer block coordinates in a dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Returns the containing horizontal chunk and the block's local X/Z.
    #[must_use]
    pub fn chunk_address(self) -> ChunkAddress {
        ChunkAddress {
            chunk: ChunkPos::new(
                self.x.div_euclid(CHUNK_WIDTH),
                self.z.div_euclid(CHUNK_WIDTH),
            ),
            local_x: self.x.rem_euclid(CHUNK_WIDTH) as u8,
            local_z: self.z.rem_euclid(CHUNK_WIDTH) as u8,
        }
    }

    /// Returns the containing 16×16×16 section and local X/Y/Z coordinates.
    ///
    /// This is a geometric address only; it does not define chunk serialization,
    /// palette storage, dimension height, or block-state encoding.
    #[must_use]
    pub fn section_address(self) -> SectionAddress {
        SectionAddress {
            section: SectionPos::new(
                self.x.div_euclid(CHUNK_WIDTH),
                self.y.div_euclid(CHUNK_WIDTH),
                self.z.div_euclid(CHUNK_WIDTH),
            ),
            local_x: self.x.rem_euclid(CHUNK_WIDTH) as u8,
            local_y: self.y.rem_euclid(CHUNK_WIDTH) as u8,
            local_z: self.z.rem_euclid(CHUNK_WIDTH) as u8,
        }
    }
}

/// Horizontal chunk coordinates. No dimension height is implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    #[must_use]
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

/// A chunk coordinate plus a valid horizontal local coordinate.
///
/// The local fields are private so values can only be created through the
/// block-to-chunk conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkAddress {
    chunk: ChunkPos,
    local_x: u8,
    local_z: u8,
}

impl ChunkAddress {
    #[must_use]
    pub const fn chunk(self) -> ChunkPos {
        self.chunk
    }

    #[must_use]
    pub const fn local_x(self) -> u8 {
        self.local_x
    }

    #[must_use]
    pub const fn local_z(self) -> u8 {
        self.local_z
    }

    /// Reconstructs a block position using the supplied absolute Y coordinate.
    #[must_use]
    pub fn to_block_pos(self, y: i32) -> Option<BlockPos> {
        let x = self
            .chunk
            .x
            .checked_mul(CHUNK_WIDTH)?
            .checked_add(i32::from(self.local_x))?;
        let z = self
            .chunk
            .z
            .checked_mul(CHUNK_WIDTH)?
            .checked_add(i32::from(self.local_z))?;

        Some(BlockPos::new(x, y, z))
    }
}

/// Three-dimensional section coordinates. No dimension height is implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectionPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl SectionPos {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

/// A section coordinate plus valid local coordinates in the 0..16 range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectionAddress {
    section: SectionPos,
    local_x: u8,
    local_y: u8,
    local_z: u8,
}

impl SectionAddress {
    #[must_use]
    pub const fn section(self) -> SectionPos {
        self.section
    }

    #[must_use]
    pub const fn local_x(self) -> u8 {
        self.local_x
    }

    #[must_use]
    pub const fn local_y(self) -> u8 {
        self.local_y
    }

    #[must_use]
    pub const fn local_z(self) -> u8 {
        self.local_z
    }

    /// Reconstructs the absolute block position, returning None on integer overflow.
    #[must_use]
    pub fn to_block_pos(self) -> Option<BlockPos> {
        let x = self.section.x.checked_mul(CHUNK_WIDTH)?.checked_add(i32::from(self.local_x))?;
        let y = self.section.y.checked_mul(CHUNK_WIDTH)?.checked_add(i32::from(self.local_y))?;
        let z = self.section.z.checked_mul(CHUNK_WIDTH)?.checked_add(i32::from(self.local_z))?;
        Some(BlockPos::new(x, y, z))
    }
}

#[cfg(test)]
mod section_tests {
    use super::*;

    #[test]
    fn section_address_uses_euclidean_division_on_all_axes() {
        let address = BlockPos::new(-1, -17, 16).section_address();
        assert_eq!(address.section(), SectionPos::new(-1, -2, 1));
        assert_eq!((address.local_x(), address.local_y(), address.local_z()), (15, 15, 0));
    }

    #[test]
    fn section_address_round_trips_boundaries() {
        for block in [
            BlockPos::new(-17, -16, -1),
            BlockPos::new(-16, -1, 0),
            BlockPos::new(15, 16, 31),
            BlockPos::new(i32::MAX, i32::MIN, 0),
        ] {
            assert_eq!(block.section_address().to_block_pos(), Some(block));
        }
    }
}
