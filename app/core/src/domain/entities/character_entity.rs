use std::fmt;

pub enum CharacterAlternativeDisplay {
	None,
	Sequence(String),
	Picture,
}

pub struct CharacterEntity {
	pub id: u32,
	pub name: String,
	pub code_point: Vec<u32>,
	pub metadata: CharacterMetadata,
	pub unicode_data: CharacterUnicodeData,
	pub composition: CharacterComposition,
	pub options: CharacterOptions,
	pub hotkeys: Vec<CharacterHotKey>,
	pub localization: CharacterLocalizationAttributes,
	pub alternative_display: CharacterAlternativeDisplay,
}

pub struct CharacterMetadata {
	pub tags: Vec<String>,
	pub titles: Vec<String>,
	pub description: String,
	pub groups: Vec<String>,
	pub category: String,
}

pub struct CharacterUnicodeData {
	pub block: String,
	pub script: String,
	pub general_category: String,
	pub combining_class: u8,
	pub bidirectional_category: String,
	pub decomposition_type: Option<String>,
	pub numeric_value: Option<f32>,
	pub mirrored: bool,
	pub unicode_version: String,
}

pub struct CharacterComposition {
	pub is_composite: bool,
	pub compositions: Vec<u32>,
}

pub struct CharacterOptions {
	pub is_favorite: bool,
	pub custom_name: Option<String>,
	pub exclude_from_search: bool,
	pub exclude_from_total_count: bool,
}

pub enum CharacterLocalizationPart {
	None,
	Composition(CharacterLocalizationParts),
	Single(String),
}

pub struct CharacterLocalizationAttributes {
	pub composited_name: bool,                      // if true then field below will be used to compose display name
	pub name: CharacterLocalizationParts,           // A, B, C, Alpha, Beta, Gamma, Omega, Epsilon
	pub secondary_name: CharacterLocalizationParts, // (Еры с Ерем) → Ы (Еры с Ерем)
	pub script: CharacterLocalizationPart,          // Latin, Cyrillic, Hellenic
	pub class: CharacterLocalizationPart,           // Letter, Symbol, Numeral
}

pub struct CharacterLocalizationParts {
	pub before: String,
	pub middle: String,
	pub after: String,
}

pub enum KeyboardLanguage {
	Any,
	English,
	Japanese,
	Spanish,
	French,
	German,
	Chinese,
	Russian,
	Arabic,
	Other(String),
}

pub enum ScripterMode {
	Any,
	GermanicRunes,
	Glagolitic,
	Hellenic,
}

pub struct CharacterHotKey {
	pub scripter_mode: ScripterMode,
	pub target_language: KeyboardLanguage,
	pub key_combination: String,
}

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

impl CharacterEntity {
	pub fn to_html_code(&self) -> String {
		self.code_point.iter().map(|cp| format!("&#{};", cp)).collect::<Vec<String>>().join("")
	}
	pub fn to_decimal_html_code(&self) -> String {
		self.code_point.iter().map(|cp| format!("&#x{:X};", cp)).collect::<Vec<String>>().join("")
	}
	pub fn to_decimal_code(&self) -> String {
		self.code_point.iter().map(|cp| format!("{:X}", cp)).collect::<Vec<String>>().join(" ")
	}
}

impl fmt::Display for CharacterEntity {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "CharacterEntity {{ id: {}, name: {}, description: {} }}", self.id, self.name, self.description)
	}
}
