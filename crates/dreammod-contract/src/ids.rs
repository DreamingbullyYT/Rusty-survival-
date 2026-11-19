use core::fmt;
use core::marker::PhantomData;

/// A validated lowercase identifier for a mod owner.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ModId(String);

impl ModId {
    pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
        let value = value.into();
        if !valid_token(&value, 64) {
            return Err(IdError::InvalidModId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for ModId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}

/// A namespaced resource key such as `dreammod:vehicles/small_car`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ResourceId {
    namespace: String,
    path: String,
}

impl ResourceId {
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Result<Self, IdError> {
        let namespace = namespace.into();
        let path = path.into();
        if !valid_token(&namespace, 64) || !valid_resource_path(&path) {
            return Err(IdError::InvalidResourceId);
        }
        Ok(Self { namespace, path })
    }

    pub fn namespace(&self) -> &str { &self.namespace }
    pub fn path(&self) -> &str { &self.path }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdError {
    InvalidModId,
    InvalidResourceId,
}

fn valid_token(value: &str, max_len: usize) -> bool {
    !value.is_empty()
        && value.len() <= max_len
        && value.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-' || b == b'.')
        && !value.starts_with('.')
        && !value.ends_with('.')
        && !value.contains("..")
}

fn valid_resource_path(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && !value.starts_with('/')
        && !value.ends_with('/')
        && !value.contains('\\')
        && value.split('/').all(|part| {
            !part.is_empty()
                && part != "."
                && part != ".."
                && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-' || b == b'.')
        })
}

/// Opaque engine handle. IDs are not pointers and must be validated by the host.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Handle<T = ()> {
    id: u64,
    generation: u32,
    kind: HandleKind,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    pub const fn new(id: u64, generation: u32, kind: HandleKind) -> Self {
        Self { id, generation, kind, _marker: PhantomData }
    }
    pub const fn id(self) -> u64 { self.id }
    pub const fn generation(self) -> u32 { self.generation }
    pub const fn kind(self) -> HandleKind { self.kind }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HandleKind {
    Entity,
    Chunk,
    BlockEntity,
    Resource,
    PhysicsBody,
    UiElement,
    System,
}
