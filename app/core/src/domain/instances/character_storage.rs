use crate::domain::storages::character_storage::CharacterStore;
use std::sync::{OnceLock, RwLock};

pub static CHARACTER_DB: OnceLock<RwLock<CharacterStore>> = OnceLock::new();
