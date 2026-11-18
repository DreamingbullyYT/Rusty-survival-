//! Trusted, bounded snapshot collection from Horizon's live server player service.
//!
//! This module returns owned values only. No Horizon handle is retained in a snapshot.

use mod_api::ServerHost;
use std::sync::Arc;

pub const MAX_PLAYER_SNAPSHOT_BATCH: usize = 256;
pub const MAX_PLAYER_NAME_BYTES: usize = 16;
pub const MAX_DIMENSION_ID_BYTES: usize = 255;
pub const MAX_WORLD_COORDINATE: f64 = 30_000_000.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerSnapshotError {
    TooManyPlayers { actual: usize, maximum: usize },
    PlayerUnavailable { uuid: u128 },
    InvalidPlayerName { uuid: u128 },
    InvalidDimension { uuid: u128 },
    InvalidPosition { uuid: u128 },
    DuplicatePlayer { uuid: u128 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSnapshot {
    uuid: u128,
    name: String,
    dimension: String,
    position: [f64; 3],
}

impl PlayerSnapshot {
    fn new(
        uuid: u128,
        name: String,
        dimension: String,
        position: [f64; 3],
    ) -> Result<Self, PlayerSnapshotError> {
        if name.is_empty()
            || name.len() > MAX_PLAYER_NAME_BYTES
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(PlayerSnapshotError::InvalidPlayerName { uuid });
        }

        if !valid_dimension_id(&dimension) {
            return Err(PlayerSnapshotError::InvalidDimension { uuid });
        }

        if position
            .iter()
            .any(|value| !value.is_finite() || value.abs() > MAX_WORLD_COORDINATE)
        {
            return Err(PlayerSnapshotError::InvalidPosition { uuid });
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

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSnapshotBatch {
    players: Arc<[PlayerSnapshot]>,
}

impl PlayerSnapshotBatch {
    fn new(players: Vec<PlayerSnapshot>) -> Result<Self, PlayerSnapshotError> {
        if players.len() > MAX_PLAYER_SNAPSHOT_BATCH {
            return Err(PlayerSnapshotError::TooManyPlayers {
                actual: players.len(),
                maximum: MAX_PLAYER_SNAPSHOT_BATCH,
            });
        }

        // A fixed stack buffer avoids heap allocation during bounded batch validation.
        let mut uuids = [0_u128; MAX_PLAYER_SNAPSHOT_BATCH];
        for (slot, player) in uuids.iter_mut().zip(&players) {
            *slot = player.uuid();
        }
        let uuids = &mut uuids[..players.len()];
        uuids.sort_unstable();
        if let Some(duplicate) = uuids.windows(2).find(|pair| pair[0] == pair[1]) {
            return Err(PlayerSnapshotError::DuplicatePlayer { uuid: duplicate[0] });
        }

        Ok(Self {
            players: Arc::from(players),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizedPlayerReadError<E> {
    PermissionDenied(E),
    Collection(PlayerSnapshotError),
}

/// Runs a player-read operation only after the caller's per-mod permission gate passes.
/// This helper deliberately accepts an operation closure so denial can be tested without
/// constructing a live Horizon ServerHost.
pub fn with_player_read_authorization<E, T>(
    authorize: impl FnOnce() -> Result<(), E>,
    operation: impl FnOnce() -> Result<T, PlayerSnapshotError>,
) -> Result<T, AuthorizedPlayerReadError<E>> {
    authorize().map_err(AuthorizedPlayerReadError::PermissionDenied)?;
    operation().map_err(AuthorizedPlayerReadError::Collection)
}

/// Collects the server's online players in Horizon's connection order, but only after
/// the supplied per-mod lifecycle adapter confirms entities.read.
/// The gate is checked once for the entire bounded batch, before touching ServerHost.
pub fn collect_online_players<E>(
    host: &mut dyn ServerHost,
    authorize: impl FnOnce() -> Result<(), E>,
) -> Result<PlayerSnapshotBatch, AuthorizedPlayerReadError<E>> {
    with_player_read_authorization(authorize, || collect_online_players_unchecked(host))
}

/// Internal collector. Never expose this directly to the script/runtime layer.
fn collect_online_players_unchecked(
    host: &mut dyn ServerHost,
) -> Result<PlayerSnapshotBatch, PlayerSnapshotError> {
    let players = host.players().server_players();
    if players.len() > MAX_PLAYER_SNAPSHOT_BATCH {
        return Err(PlayerSnapshotError::TooManyPlayers {
            actual: players.len(),
            maximum: MAX_PLAYER_SNAPSHOT_BATCH,
        });
    }

    let mut snapshots = Vec::with_capacity(players.len());
    for player in players {
        let uuid = player.uuid();
        let name = player
            .name(host)
            .ok_or(PlayerSnapshotError::PlayerUnavailable { uuid })?;
        let dimension = player
            .dimension(host)
            .map(|identifier| identifier.to_string())
            .ok_or(PlayerSnapshotError::PlayerUnavailable { uuid })?;
        let position = player
            .entity()
            .position(host)
            .ok_or(PlayerSnapshotError::PlayerUnavailable { uuid })?;

        snapshots.push(PlayerSnapshot::new(uuid, name, dimension, position)?);
    }

    PlayerSnapshotBatch::new(snapshots)
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
        && namespace.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.')
        })
        && path.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'_' | b'-' | b'.' | b'/')
        })
}

#[cfg(test)]
mod authorization_tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn denied_player_read_never_invokes_host_operation() {
        let calls = Cell::new(0);
        let result = with_player_read_authorization(
            || Err("entities.read denied"),
            || {
                calls.set(calls.get() + 1);
                Ok(())
            },
        );
        assert_eq!(
            result,
            Err(AuthorizedPlayerReadError::PermissionDenied(
                "entities.read denied"
            ))
        );
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn authorized_player_read_invokes_operation_once() {
        let calls = Cell::new(0);
        let result = with_player_read_authorization(
            || Ok::<(), &'static str>(()),
            || {
                calls.set(calls.get() + 1);
                Ok(42)
            },
        );
        assert_eq!(result, Ok(42));
        assert_eq!(calls.get(), 1);
    }
    fn valid_snapshot(uuid: u128) -> PlayerSnapshot {
        PlayerSnapshot::new(
            uuid,
            "Dreamer_7".into(),
            "minecraft:overworld".into(),
            [1.0, 64.0, -2.0],
        )
        .unwrap()
    }

    #[test]
    fn snapshot_rejects_invalid_player_names() {
        for name in ["", "player name", "name!", "é", "a-very-long-name-over-16"] {
            assert!(matches!(
                PlayerSnapshot::new(1, name.into(), "minecraft:overworld".into(), [0.0; 3]),
                Err(PlayerSnapshotError::InvalidPlayerName { uuid: 1 })
            ));
        }
    }

    #[test]
    fn snapshot_rejects_invalid_dimension_identifiers() {
        for dimension in [
            "",
            "overworld",
            ":path",
            "minecraft:",
            "Minecraft:overworld",
            "minecraft:a:b",
        ] {
            assert!(matches!(
                PlayerSnapshot::new(1, "Dreamer".into(), dimension.into(), [0.0; 3]),
                Err(PlayerSnapshotError::InvalidDimension { uuid: 1 })
            ));
        }
    }

    #[test]
    fn snapshot_rejects_non_finite_and_out_of_range_positions() {
        for position in [
            [f64::NAN, 0.0, 0.0],
            [0.0, f64::INFINITY, 0.0],
            [0.0, 0.0, -30_000_001.0],
        ] {
            assert!(matches!(
                PlayerSnapshot::new(1, "Dreamer".into(), "minecraft:overworld".into(), position),
                Err(PlayerSnapshotError::InvalidPosition { uuid: 1 })
            ));
        }
    }

    #[test]
    fn snapshot_batch_clone_shares_immutable_storage() {
        let batch = PlayerSnapshotBatch::new(vec![valid_snapshot(7)]).unwrap();
        let clone = batch.clone();
        assert!(Arc::ptr_eq(&batch.players, &clone.players));
    }

    #[test]
    fn snapshot_batch_rejects_duplicate_player_ids() {
        let result = PlayerSnapshotBatch::new(vec![valid_snapshot(7), valid_snapshot(7)]);
        assert_eq!(
            result,
            Err(PlayerSnapshotError::DuplicatePlayer { uuid: 7 })
        );
    }

    #[test]
    fn snapshot_batch_enforces_the_bounded_size() {
        let players = (0..=MAX_PLAYER_SNAPSHOT_BATCH)
            .map(|uuid| valid_snapshot(uuid as u128))
            .collect();
        assert_eq!(
            PlayerSnapshotBatch::new(players),
            Err(PlayerSnapshotError::TooManyPlayers {
                actual: MAX_PLAYER_SNAPSHOT_BATCH + 1,
                maximum: MAX_PLAYER_SNAPSHOT_BATCH,
            })
        );
    }
}
