//! Owned, bounded values crossing from Horizon into interpreted DreamMod code.
//!
//! These values deliberately contain no Horizon handles, references or borrowed strings.

pub use dreammod_contract::PLAYER_READ_CAPABILITY;
pub const MAX_PLAYER_NAME_BYTES: usize = 16;
pub const MAX_PLAYER_SNAPSHOT_BATCH: usize = 256;
pub const MAX_DIMENSION_ID_BYTES: usize = 255;
pub const MAX_WORLD_COORDINATE: f64 = 30_000_000.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotError {
    InvalidPlayerName,
    InvalidDimensionId,
    InvalidPosition,
    BatchTooLarge,
    DuplicatePlayer(u128),
}

/// A small, owned player view. It is safe to retain after the Horizon hook returns.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSnapshot {
    uuid: u128,
    name: String,
    dimension: String,
    position: [f64; 3],
}

impl PlayerSnapshot {
    pub fn new(
        uuid: u128,
        name: String,
        dimension: String,
        position: [f64; 3],
    ) -> Result<Self, SnapshotError> {
        if name.is_empty()
            || name.len() > MAX_PLAYER_NAME_BYTES
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(SnapshotError::InvalidPlayerName);
        }

        if !valid_dimension_id(&dimension) {
            return Err(SnapshotError::InvalidDimensionId);
        }

        if position.iter().any(|value| {
            !value.is_finite() || value.abs() > MAX_WORLD_COORDINATE
        }) {
            return Err(SnapshotError::InvalidPosition);
        }

        Ok(Self {
            uuid,
            name,
            dimension,
            position,
        })
    }

    pub fn uuid(&self) -> u128 {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dimension(&self) -> &str {
        &self.dimension
    }

    pub fn position(&self) -> [f64; 3] {
        self.position
    }
}

/// A bounded batch for one script/native boundary crossing.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSnapshotBatch {
    players: std::sync::Arc<[PlayerSnapshot]>,
}

impl PlayerSnapshotBatch {
    pub fn new(players: Vec<PlayerSnapshot>) -> Result<Self, SnapshotError> {
        if players.len() > MAX_PLAYER_SNAPSHOT_BATCH {
            return Err(SnapshotError::BatchTooLarge);
        }

        // A fixed stack buffer avoids heap allocation during bounded batch validation.
        let mut uuids = [0_u128; MAX_PLAYER_SNAPSHOT_BATCH];
        for (slot, player) in uuids.iter_mut().zip(&players) {
            *slot = player.uuid();
        }
        let uuids = &mut uuids[..players.len()];
        uuids.sort_unstable();
        if let Some(duplicate) = uuids.windows(2).find(|pair| pair[0] == pair[1]) {
            return Err(SnapshotError::DuplicatePlayer(duplicate[0]));
        }

        Ok(Self {
            players: std::sync::Arc::from(players),
        })
    }

    pub fn players(&self) -> &[PlayerSnapshot] {
        &self.players
    }

    pub fn len(&self) -> usize {
        self.players.len()
    }

    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }
}

fn valid_dimension_id(value: &str) -> bool {
    if value.is_empty() || value.len() > MAX_DIMENSION_ID_BYTES {
        return false;
    }
    let Some((namespace, path)) = value.split_once(':') else {
        return false;
    };
    !namespace.is_empty()
        && !path.is_empty()
        && !path.contains(':')
        && namespace
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.'))
        && path.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'_' | b'-' | b'.' | b'/')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_bounded_owned_snapshot() {
        let snapshot = PlayerSnapshot::new(
            7,
            "Dreamer_7".into(),
            "minecraft:overworld".into(),
            [12.5, 64.0, -3.25],
        )
        .unwrap();

        assert_eq!(snapshot.uuid(), 7);
        assert_eq!(snapshot.name(), "Dreamer_7");
        assert_eq!(snapshot.dimension(), "minecraft:overworld");
        assert_eq!(snapshot.position(), [12.5, 64.0, -3.25]);
    }

    #[test]
    fn snapshot_batch_clone_shares_immutable_storage() {
        let player = PlayerSnapshot::new(
            7,
            "Dreamer_7".into(),
            "minecraft:overworld".into(),
            [0.0; 3],
        )
        .unwrap();
        let batch = PlayerSnapshotBatch::new(vec![player]).unwrap();
        let clone = batch.clone();
        assert!(std::sync::Arc::ptr_eq(&batch.players, &clone.players));
    }

    #[test]
    fn snapshot_batches_are_bounded_and_reject_duplicate_players() {
        let player = || {
            PlayerSnapshot::new(7, "Dreamer_7".into(), "minecraft:overworld".into(), [0.0; 3]).unwrap()
        };
        let batch = PlayerSnapshotBatch::new(vec![player()]).unwrap();
        assert_eq!(batch.len(), 1);
        assert_eq!(batch.players()[0].uuid(), 7);

        assert_eq!(
            PlayerSnapshotBatch::new(vec![player(), player()]),
            Err(SnapshotError::DuplicatePlayer(7))
        );
        assert_eq!(
            PlayerSnapshotBatch::new(vec![player(); MAX_PLAYER_SNAPSHOT_BATCH + 1]),
            Err(SnapshotError::BatchTooLarge)
        );
    }

    #[test]
    fn rejects_invalid_names_dimensions_and_positions() {
        assert_eq!(
            PlayerSnapshot::new(1, "name with spaces".into(), "minecraft:overworld".into(), [0.0; 3]),
            Err(SnapshotError::InvalidPlayerName)
        );
        assert_eq!(
            PlayerSnapshot::new(1, "ValidName".into(), "overworld".into(), [0.0; 3]),
            Err(SnapshotError::InvalidDimensionId)
        );
        assert_eq!(
            PlayerSnapshot::new(1, "ValidName".into(), "minecraft:overworld".into(), [f64::NAN, 0.0, 0.0]),
            Err(SnapshotError::InvalidPosition)
        );
        assert_eq!(
            PlayerSnapshot::new(1, "ValidName".into(), "minecraft:overworld".into(), [30_000_001.0, 0.0, 0.0]),
            Err(SnapshotError::InvalidPosition)
        );
    }
}
