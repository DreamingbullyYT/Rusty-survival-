//! Trusted, bounded snapshot collection from Horizon's live server player service.
//!
//! This module returns owned values only. No Horizon handle is retained in a snapshot.

use mod_api::ServerHost;
use std::collections::BTreeSet;

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

        if position.iter().any(|value| {
            !value.is_finite() || value.abs() > MAX_WORLD_COORDINATE
        }) {
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
    players: Vec<PlayerSnapshot>,
}

impl PlayerSnapshotBatch {
    fn new(players: Vec<PlayerSnapshot>) -> Result<Self, PlayerSnapshotError> {
        if players.len() > MAX_PLAYER_SNAPSHOT_BATCH {
            return Err(PlayerSnapshotError::TooManyPlayers {
                actual: players.len(),
                maximum: MAX_PLAYER_SNAPSHOT_BATCH,
            });
        }

        let mut seen = BTreeSet::new();
        for player in &players {
            if !seen.insert(player.uuid()) {
                return Err(PlayerSnapshotError::DuplicatePlayer {
                    uuid: player.uuid(),
                });
            }
        }

        Ok(Self { players })
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

/// Collects the server's online players in Horizon's connection order.
///
/// The player list is capped before any per-player reads. Every Horizon handle is
/// used only inside this function; returned snapshots own all their data.
pub fn collect_online_players(
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
        && namespace
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.'))
        && path.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'_' | b'-' | b'.' | b'/')
        })
}
