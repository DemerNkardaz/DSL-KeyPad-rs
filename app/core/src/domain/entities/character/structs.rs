use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character {
	pub internal_id: u64,
	pub name: String,
	pub keyboard_binds: Option<SmallVec<[KeyboardBind; 4]>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardBind {
	pub key: String,
	pub modifiers: Option<SmallVec<[String; 4]>>,
	pub target_language: String,
	pub mode: String,
}
