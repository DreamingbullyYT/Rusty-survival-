//! Typed world/block command adapter. Native backends own authoritative state and atomicity.
use crate::runtime::{AuthorizedCommand, EngineCommandHost};
use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BlockId {
    namespace: String,
    path: String,
}

impl BlockId {
    pub fn parse(value: &str) -> Result<Self, BlockAdapterError<()>> {
        let (namespace, path) = value
            .split_once(':')
            .ok_or(BlockAdapterError::InvalidBlockId)?;
        if !valid_namespace(namespace) || !valid_path(path) {
            return Err(BlockAdapterError::InvalidBlockId);
        }
        Ok(Self {
            namespace: namespace.to_owned(),
            path: path.to_owned(),
        })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

fn valid_namespace(v: &str) -> bool {
    !v.is_empty()
        && v.len() <= 64
        && v.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-' || b == b'.'
        })
        && !v.starts_with('.')
        && !v.ends_with('.')
        && !v.contains("..")
}

fn valid_path(v: &str) -> bool {
    !v.is_empty()
        && v.len() <= 256
        && !v.starts_with('/')
        && !v.ends_with('/')
        && !v.contains('\\')
        && v.split('/').all(|p| {
            !p.is_empty()
                && p != "."
                && p != ".."
                && p.bytes().all(|b| {
                    b.is_ascii_lowercase()
                        || b.is_ascii_digit()
                        || b == b'_'
                        || b == b'-'
                        || b == b'.'
                })
        })
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPosition {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockMutation {
    pub position: BlockPosition,
    pub block: BlockId,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SetBlockPayload {
    x: i32,
    y: i32,
    z: i32,
    block: String,
}

/// Implementations MUST stage all mutations and commit atomically, or leave the world unchanged on error.
pub trait BlockWorldBackend {
    type Error;

    fn set_blocks_atomic(&mut self, mutations: &[BlockMutation]) -> Result<(), Self::Error>;
}

#[derive(Debug, Error)]
pub enum BlockAdapterError<E> {
    #[error("unsupported command: {0}")]
    UnsupportedCommand(String),
    #[error("invalid set_block payload")]
    InvalidPayload,
    #[error("invalid block identifier")]
    InvalidBlockId,
    #[error("horizontal position outside world border")]
    OutOfBounds,
    #[error("native backend failed")]
    Backend(E),
}

pub struct WorldBlockCommandAdapter<B> {
    backend: B,
}

impl<B> WorldBlockCommandAdapter<B> {
    pub const fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn into_backend(self) -> B {
        self.backend
    }
}

impl<B: BlockWorldBackend> EngineCommandHost for WorldBlockCommandAdapter<B> {
    type Error = BlockAdapterError<B::Error>;

    fn execute_atomic_batch(&mut self, commands: &[AuthorizedCommand]) -> Result<(), Self::Error> {
        let mut staged = Vec::with_capacity(commands.len());
        for c in commands {
            let cmd = &c.command;
            if cmd.namespace != "world" || cmd.name != "set_block" {
                return Err(BlockAdapterError::UnsupportedCommand(format!(
                    "{}.{}",
                    cmd.namespace, cmd.name
                )));
            }
            let p: SetBlockPayload = serde_json::from_value(cmd.payload.clone())
                .map_err(|_| BlockAdapterError::InvalidPayload)?;
            if !(-30_000_000..=30_000_000).contains(&p.x)
                || !(-30_000_000..=30_000_000).contains(&p.z)
            {
                return Err(BlockAdapterError::OutOfBounds);
            }
            let block = BlockId::parse(&p.block).map_err(|_| BlockAdapterError::InvalidBlockId)?;
            staged.push(BlockMutation {
                position: BlockPosition::new(p.x, p.y, p.z),
                block,
            });
        }
        self.backend
            .set_blocks_atomic(&staged)
            .map_err(BlockAdapterError::Backend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{CommandEnvelope, ModId};
    use serde_json::json;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct Mock {
        blocks: BTreeMap<BlockPosition, BlockId>,
    }

    impl BlockWorldBackend for Mock {
        type Error = ();

        fn set_blocks_atomic(&mut self, ms: &[BlockMutation]) -> Result<(), ()> {
            let mut next = self.blocks.clone();
            for m in ms {
                next.insert(m.position, m.block.clone());
            }
            self.blocks = next;
            Ok(())
        }
    }

    fn cmd(payload: serde_json::Value) -> AuthorizedCommand {
        AuthorizedCommand {
            mod_id: ModId::parse("test.mod").unwrap(),
            command: CommandEnvelope {
                namespace: "world".into(),
                name: "set_block".into(),
                payload,
            },
        }
    }

    #[test]
    fn applies_typed_batch_and_supports_mod_block_ids() {
        let mut a = WorldBlockCommandAdapter::new(Mock::default());
        a.execute_atomic_batch(&[
            cmd(json!({"x":-1,"y":90,"z":16,"block":"minecraft:stone"})),
            cmd(json!({"x":0,"y":90,"z":0,"block":"test_mod:decor/blue_tile"})),
        ])
        .unwrap();
        assert_eq!(
            a.backend()
                .blocks
                .get(&BlockPosition::new(-1, 90, 16))
                .unwrap()
                .to_string(),
            "minecraft:stone"
        );
        assert_eq!(
            a.backend()
                .blocks
                .get(&BlockPosition::new(0, 90, 0))
                .unwrap()
                .to_string(),
            "test_mod:decor/blue_tile"
        );
    }

    #[test]
    fn validates_entire_batch_before_backend_mutation() {
        let mut a = WorldBlockCommandAdapter::new(Mock::default());
        let batch = [
            cmd(json!({"x":1,"y":2,"z":3,"block":"minecraft:stone"})),
            cmd(json!({"x":4,"y":5,"z":6,"block":"../bad"})),
        ];
        assert!(matches!(
            a.execute_atomic_batch(&batch),
            Err(BlockAdapterError::InvalidBlockId)
        ));
        assert!(a.backend().blocks.is_empty());
    }

    #[test]
    fn rejects_unknown_fields_out_of_border_and_non_world_commands() {
        let mut a = WorldBlockCommandAdapter::new(Mock::default());
        assert!(matches!(
            a.execute_atomic_batch(&[cmd(
                json!({"x":0,"y":0,"z":0,"block":"minecraft:stone","extra":true})
            )]),
            Err(BlockAdapterError::InvalidPayload)
        ));
        assert!(matches!(
            a.execute_atomic_batch(&[cmd(
                json!({"x":30_000_001,"y":0,"z":0,"block":"minecraft:stone"})
            )]),
            Err(BlockAdapterError::OutOfBounds)
        ));
        let mut e = cmd(json!({"x":0,"y":0,"z":0,"block":"minecraft:stone"}));
        e.command.namespace = "entity".into();
        e.command.name = "set_health".into();
        assert!(matches!(
            a.execute_atomic_batch(&[e]),
            Err(BlockAdapterError::UnsupportedCommand(_))
        ));
        assert!(a.backend().blocks.is_empty());
    }
}
