//! Bounded, owner-scoped atlas registration batches for the trusted Core.
//!
//! Script-facing adapters must check the mod's capability before calling this API.
//! The Horizon hook only reads an owned snapshot and re-submits it on every reload.
use mod_api::TextureAtlasRegistration;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

pub const MAX_ATLASES: usize = 32;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtlasRegistryError<E> {
    Authorization(E), InvalidOwner,
    NamespaceMismatch { owner: String, identifier: String },
    UnsafePath(String), DuplicateAtlas(String), LimitExceeded { maximum: usize },
}
type Registry = BTreeMap<String, Vec<TextureAtlasRegistration>>;
static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();
fn registry() -> &'static Mutex<Registry> { REGISTRY.get_or_init(|| Mutex::new(BTreeMap::new())) }
fn lock_registry() -> std::sync::MutexGuard<'static, Registry> {
    registry().lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
fn valid_owner(owner: &str) -> bool {
    owner.len() <= 128 && owner.split('.').count() >= 2 && owner.split('.').all(|part| {
        !part.is_empty() && !part.starts_with('-') && !part.ends_with('-')
            && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    })
}
fn valid_path(path: &str) -> bool {
    !path.is_empty() && !path.contains('\\') && path.split('/').all(|part| {
        !part.is_empty() && part != "." && part != ".."
            && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.'))
    })
}
fn validate_batch<E>(owner: &str, batch: &[TextureAtlasRegistration]) -> Result<(), AtlasRegistryError<E>> {
    if !valid_owner(owner) { return Err(AtlasRegistryError::InvalidOwner); }
    let mut ids = BTreeSet::new();
    for entry in batch {
        for identifier in [&entry.atlas, &entry.definition] {
            if identifier.namespace() != owner || owner == "minecraft" || owner == "horizon" {
                return Err(AtlasRegistryError::NamespaceMismatch { owner: owner.to_owned(), identifier: identifier.to_string() });
            }
            if !valid_path(identifier.path()) { return Err(AtlasRegistryError::UnsafePath(identifier.to_string())); }
        }
        let id = entry.atlas.to_string();
        if !ids.insert(id.clone()) { return Err(AtlasRegistryError::DuplicateAtlas(id)); }
    }
    Ok(())
}
/// Atomically replace a mod's full atlas batch. The authorization callback is checked first.
/// An empty batch removes the owner. Script adapters must supply their capability gate.
pub fn replace_atlas_batch<E>(
    owner: &str, batch: Vec<TextureAtlasRegistration>, authorize: impl FnOnce() -> Result<(), E>,
) -> Result<(), AtlasRegistryError<E>> {
    authorize().map_err(AtlasRegistryError::Authorization)?;
    validate_batch(owner, &batch)?;
    let mut current = lock_registry();
    let mut prospective = current.clone();
    if batch.is_empty() { prospective.remove(owner); } else { prospective.insert(owner.to_owned(), batch); }
    let total = prospective.values().map(Vec::len).sum::<usize>();
    if total > MAX_ATLASES { return Err(AtlasRegistryError::LimitExceeded { maximum: MAX_ATLASES }); }
    let mut atlas_ids = BTreeSet::new();
    for values in prospective.values() {
        for entry in values {
            let id = entry.atlas.to_string();
            if !atlas_ids.insert(id.clone()) { return Err(AtlasRegistryError::DuplicateAtlas(id)); }
        }
    }
    *current = prospective;
    Ok(())
}
/// Snapshot all registered declarations in deterministic identifier order.
pub fn snapshot() -> Vec<TextureAtlasRegistration> {
    let mut result = lock_registry().values().flat_map(|batch| batch.iter().cloned()).collect::<Vec<_>>();
    result.sort_by(|a, b| a.atlas.to_string().cmp(&b.atlas.to_string()));
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use mod_api::Identifier;
    fn entry(owner: &str, atlas: &str, definition: &str) -> TextureAtlasRegistration {
        TextureAtlasRegistration { atlas: Identifier::parse_namespaced(atlas).unwrap(), definition: Identifier::parse_namespaced(definition).unwrap() }
    }
    #[test]
    fn authorization_denial_does_not_mutate_registry() {
        let owner = "dreammod.test-denied";
        let result = replace_atlas_batch(owner, vec![entry(owner, "dreammod.test-denied:icons", "dreammod.test-denied:icons")], || Err("denied"));
        assert_eq!(result, Err(AtlasRegistryError::Authorization("denied")));
        assert!(snapshot().iter().all(|item| item.atlas.namespace() != owner));
    }
    #[test]
    fn replacement_is_atomic_sorted_and_removable() {
        let owner = "dreammod.test-batch";
        replace_atlas_batch(owner, vec![entry(owner, "dreammod.test-batch:z", "dreammod.test-batch:z"), entry(owner, "dreammod.test-batch:a", "dreammod.test-batch:a")], || Ok::<(), ()>(())).unwrap();
        let owned=snapshot().into_iter().filter(|item| item.atlas.namespace()==owner).collect::<Vec<_>>();
        assert_eq!(owned.iter().map(|item|item.atlas.path()).collect::<Vec<_>>(),vec!["a","z"]);
        let oversized=(0..=MAX_ATLASES).map(|i|entry(owner,&format!("{owner}:x{i}"),&format!("{owner}:x{i}"))).collect();
        assert_eq!(replace_atlas_batch(owner,oversized,||Ok::<(),()>(())),Err(AtlasRegistryError::LimitExceeded{maximum:MAX_ATLASES}));
        assert_eq!(snapshot().into_iter().filter(|item|item.atlas.namespace()==owner).count(),2);
        replace_atlas_batch(owner,Vec::new(),||Ok::<(),()>(() )).unwrap();
        assert!(snapshot().iter().all(|item|item.atlas.namespace()!=owner));
    }
    #[test]
    fn foreign_namespace_and_traversal_are_rejected() {
        let owner="dreammod.test-path";
        assert!(matches!(replace_atlas_batch(owner,vec![entry(owner,"other:icons","other:icons")],||Ok::<(),()>(())),Err(AtlasRegistryError::NamespaceMismatch{..})));
        assert!(matches!(replace_atlas_batch(owner,vec![entry(owner,"dreammod.test-path:icons","dreammod.test-path:../bad")],||Ok::<(),()>(())),Err(AtlasRegistryError::UnsafePath(_))));
    }
}
