use smallvec::SmallVec;
use std::sync::Arc;

use super::enums::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterEntity {
	pub id: u32,
	pub metadata: CharacterMetadata,
	pub unicode_data: CharacterUnicodeData,
	pub composition: CharacterComposition,
	pub options: CharacterOptions,
	pub hotkeys: Option<Vec<CharacterHotKey>>,
	pub localization: CharacterLocalizationAttributes,
	pub alternative_display: CharacterAlternativeDisplay,
	pub children: Option<Vec<CharacterEntity>>, // Superscript, Subscript, Fraktur, Small Capital
}
 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterMetadata {
	pub tags: Option<Vec<String>>,
	pub titles: Option<Vec<String>>,
	pub description: Option<String>,
	pub groups: Vec<Arc<str>>,
	pub category: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterUnicodeData {
	pub code_point: SmallVec<[u32; 1]>,
	pub name: String,
	pub block: String,
	pub script: String,
	pub general_category: String,
	pub combining_class: u8,
	pub bidirectional_category: String,
	pub decomposition_type: Option<String>,
	pub numeric_value: Option<u32>,
	pub mirrored: bool,
	pub unicode_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterComposition {
	pub compositions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterOptions {
	pub is_favorite: bool,
	pub custom_name: Option<String>,
	pub exclude_from_search: bool,
	pub exclude_from_total_count: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterLocalizationAttributes {
	pub composited_name: bool,                              // if true then field below will be used to compose display name
	pub name: Option<CharacterLocalizationParts>,           // A, B, C, Alpha, Beta, Gamma, Omega, Epsilon
	pub secondary_name: Option<CharacterLocalizationParts>, // (Еры с Ерем) → Ы (Еры с Ерем)
	pub script: Option<CharacterLocalizationPart>,          // Latin, Cyrillic, Hellenic
	pub class: Option<CharacterLocalizationPart>,           // Letter, Symbol, Numeral
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterLocalizationParts {
	pub before: Option<String>,
	pub middle: String,
	pub after: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterHotKey {
	pub scripter_mode: Option<ScripterMode>,
	pub mode: Option<HotkeysMode>,
	pub target_language: KeyboardLanguage,
	pub key_combination: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterDisplay {
	pub character: CharacterEntity,
	pub font_family: String,
	pub font_size: u32,
	pub color: String,
	pub is_bold: bool,
	pub is_italic: bool,
	pub is_underlined: bool,
	pub inline_css: Option<String>,
}
