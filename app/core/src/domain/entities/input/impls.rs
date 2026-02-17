use super::{Lock, LockKey, LockState, Modifier, ModifierKey, ModifierLocation};

impl Default for ModifierLocation {
	fn default() -> Self {
		Self::Any
	}
}

impl Default for LockState {
	fn default() -> Self {
		Self::Unlocked
	}
}

impl Modifier {
	pub fn shift(location: ModifierLocation) -> Self {
		Self { key: ModifierKey::Shift, location }
	}

	pub fn control(location: ModifierLocation) -> Self {
		Self { key: ModifierKey::Control, location }
	}

	pub fn alt(location: ModifierLocation) -> Self {
		Self { key: ModifierKey::Alt, location }
	}

	pub fn meta() -> Self {
		Self {
			key: ModifierKey::Meta,
			location: ModifierLocation::Any,
		}
	}

	pub fn match_key(&self, key: ModifierKey) -> bool {
		self.key == key
	}

	pub fn match_location(&self, location: ModifierLocation) -> bool {
		self.location == location
	}
}

impl Lock {
	pub fn numlock(state: LockState) -> Self {
		Self { key: LockKey::NumLock, state }
	}

	pub fn capslock(state: LockState) -> Self {
		Self { key: LockKey::CapsLock, state }
	}

	pub fn scrolllock(state: LockState) -> Self {
		Self { key: LockKey::ScrollLock, state }
	}

	pub fn is_locked(&self) -> bool {
		self.state == LockState::Locked
	}
}
