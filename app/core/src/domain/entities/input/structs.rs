use super::{LockKey, LockState, ModifierKey, ModifierLocation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifier {
	pub key: ModifierKey,
	pub location: ModifierLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lock {
	pub key: LockKey,
	pub state: LockState,
}
