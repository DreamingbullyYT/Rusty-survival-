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
