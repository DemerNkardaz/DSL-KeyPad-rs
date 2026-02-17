#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierKey {
	Shift,
	Control,
	Alt,  // Alt(Right) equals both to the RAlt and AltGR keys
	Meta, // also Windows
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierLocation {
	Any,
	Left,
	Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LockKey {
	NumLock,
	CapsLock,
	ScrollLock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LockState {
	Locked,
	Unlocked,
}
