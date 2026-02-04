use smallvec::SmallVec;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterAlternativeDisplay {
	None,
	Sequence(String),
	Picture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterEntity {
	pub id: u32,
	pub metadata: CharacterMetadata,
	pub unicode_data: CharacterUnicodeData,
	pub composition: CharacterComposition,
	pub options: CharacterOptions,
	pub hotkeys: Vec<CharacterHotKey>,
	pub localization: CharacterLocalizationAttributes,
	pub alternative_display: CharacterAlternativeDisplay,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterMetadata {
	pub tags: Vec<String>,
	pub titles: Vec<String>,
	pub description: String,
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
	pub compositions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterOptions {
	pub is_favorite: bool,
	pub custom_name: Option<String>,
	pub exclude_from_search: bool,
	pub exclude_from_total_count: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterLocalizationPart {
	None,
	Composition(CharacterLocalizationParts),
	Single(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterLocalizationAttributes {
	pub composited_name: bool,                      // if true then field below will be used to compose display name
	pub name: CharacterLocalizationParts,           // A, B, C, Alpha, Beta, Gamma, Omega, Epsilon
	pub secondary_name: CharacterLocalizationParts, // (Еры с Ерем) → Ы (Еры с Ерем)
	pub script: CharacterLocalizationPart,          // Latin, Cyrillic, Hellenic
	pub class: CharacterLocalizationPart,           // Letter, Symbol, Numeral
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterLocalizationParts {
	pub before: String,
	pub middle: String,
	pub after: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScripterMode {
	Any,
	GermanicRunes,
	Glagolitic,
	Hellenic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterHotKey {
	pub scripter_mode: ScripterMode,
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

#[allow(clippy::must_use_candidate)]
impl CharacterEntity {
	pub fn to_html_code(&self) -> String {
		let mut result = String::new();
		self.unicode_data.code_point.iter().for_each(|cp| result.extend(format!("&#x{cp:X};").chars()));
		result
	}

	pub fn to_decimal_html_code(&self) -> String {
		let mut result = String::new();
		self.unicode_data.code_point.iter().for_each(|cp| result.extend(format!("&#{cp};").chars()));
		result
	}

	pub fn as_hex(&self) -> Vec<String> {
		self.unicode_data.code_point.iter().map(|cp| format!("{cp:04X}")).collect()
	}
}

impl fmt::Display for CharacterEntity {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_html_code())
	}
}

#[allow(clippy::must_use_candidate)]
impl CharacterComposition {
	pub const fn is_composite(&self) -> bool {
		self.compositions.len() > 1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_character_entity_display() {
		// Простой символ - латинская буква A
		let simple_char = CharacterEntity {
			id: 1,
			metadata: CharacterMetadata {
				tags: vec!["letter".to_string(), "uppercase".to_string()],
				titles: vec!["A".to_string()],
				description: "Latin capital letter A".to_string(),
				groups: vec!["Basic Latin".to_string().into()],
				category: "Letter".to_string(),
			},
			unicode_data: CharacterUnicodeData {
				code_point: SmallVec::from_vec(vec![0x0041]),
				name: "LATIN CAPITAL LETTER A".to_string(),
				block: "Basic Latin".to_string(),
				script: "Latin".to_string(),
				general_category: "Lu".to_string(),
				combining_class: 0,
				bidirectional_category: "L".to_string(),
				decomposition_type: None,
				numeric_value: None,
				mirrored: false,
				unicode_version: "1.1".to_string(),
			},
			composition: CharacterComposition { compositions: vec!["0041".to_string()] },
			options: CharacterOptions {
				is_favorite: true,
				custom_name: Some("My Letter A".to_string()),
				exclude_from_search: false,
				exclude_from_total_count: false,
			},
			hotkeys: vec![CharacterHotKey {
				scripter_mode: ScripterMode::Any,
				target_language: KeyboardLanguage::English,
				key_combination: "Shift+A".to_string(),
			}],
			localization: CharacterLocalizationAttributes {
				composited_name: false,
				name: CharacterLocalizationParts {
					before: "".to_string(),
					middle: "A".to_string(),
					after: "".to_string(),
				},
				secondary_name: CharacterLocalizationParts {
					before: "".to_string(),
					middle: "".to_string(),
					after: "".to_string(),
				},
				script: CharacterLocalizationPart::Single("Latin".to_string()),
				class: CharacterLocalizationPart::Single("Letter".to_string()),
			},
			alternative_display: CharacterAlternativeDisplay::None,
		};

		// Составной символ - буква с диакритикой (Á)
		let composite_char = CharacterEntity {
			id: 2,
			metadata: CharacterMetadata {
				tags: vec!["letter".to_string(), "uppercase".to_string(), "accented".to_string()],
				titles: vec!["Á".to_string()],
				description: "Latin capital letter A with acute accent".to_string(),
				groups: vec!["Latin-1 Supplement".to_string().into()],
				category: "Letter".to_string(),
			},
			unicode_data: CharacterUnicodeData {
				code_point: SmallVec::from_vec(vec![0x00C1]),
				name: "LATIN CAPITAL LETTER A WITH ACUTE".to_string(),
				block: "Latin-1 Supplement".to_string(),
				script: "Latin".to_string(),
				general_category: "Lu".to_string(),
				combining_class: 0,
				bidirectional_category: "L".to_string(),
				decomposition_type: Some("canonical".to_string()),
				numeric_value: None,
				mirrored: false,
				unicode_version: "1.1".to_string(),
			},
			composition: CharacterComposition {
				compositions: vec!["0041".to_string(), "0301".to_string()], // A + combining acute
			},
			options: CharacterOptions {
				is_favorite: false,
				custom_name: None,
				exclude_from_search: false,
				exclude_from_total_count: false,
			},
			hotkeys: vec![],
			localization: CharacterLocalizationAttributes {
				composited_name: true,
				name: CharacterLocalizationParts {
					before: "".to_string(),
					middle: "A".to_string(),
					after: " with acute".to_string(),
				},
				secondary_name: CharacterLocalizationParts {
					before: "(".to_string(),
					middle: "accented".to_string(),
					after: ")".to_string(),
				},
				script: CharacterLocalizationPart::Single("Latin".to_string()),
				class: CharacterLocalizationPart::Single("Letter".to_string()),
			},
			alternative_display: CharacterAlternativeDisplay::None,
		};

		// Эмодзи с альтернативным отображением
		let emoji_char = CharacterEntity {
			id: 3,
			metadata: CharacterMetadata {
				tags: vec!["emoji".to_string(), "face".to_string(), "smile".to_string()],
				titles: vec!["😀".to_string(), "grinning".to_string()],
				description: "A grinning face emoji".to_string(),
				groups: vec!["Emoticons".to_string().into()],
				category: "Symbol".to_string(),
			},
			unicode_data: CharacterUnicodeData {
				code_point: SmallVec::from_vec(vec![0x1F600]),
				name: "GRINNING FACE".to_string(),
				block: "Emoticons".to_string(),
				script: "Common".to_string(),
				general_category: "So".to_string(),
				combining_class: 0,
				bidirectional_category: "ON".to_string(),
				decomposition_type: None,
				numeric_value: None,
				mirrored: false,
				unicode_version: "6.1".to_string(),
			},
			composition: CharacterComposition { compositions: vec!["1F600".to_string()] },
			options: CharacterOptions {
				is_favorite: true,
				custom_name: Some("Happy Face".to_string()),
				exclude_from_search: false,
				exclude_from_total_count: false,
			},
			hotkeys: vec![CharacterHotKey {
				scripter_mode: ScripterMode::Any,
				target_language: KeyboardLanguage::Any,
				key_combination: ":)".to_string(),
			}],
			localization: CharacterLocalizationAttributes {
				composited_name: false,
				name: CharacterLocalizationParts {
					before: "".to_string(),
					middle: "Grinning Face".to_string(),
					after: "".to_string(),
				},
				secondary_name: CharacterLocalizationParts {
					before: "".to_string(),
					middle: "".to_string(),
					after: "".to_string(),
				},
				script: CharacterLocalizationPart::None,
				class: CharacterLocalizationPart::Single("Emoji".to_string()),
			},
			alternative_display: CharacterAlternativeDisplay::Picture,
		};

		// Отображение в консоли
		println!("\n=== Простой символ (A) ===");
		println!("{:#?}", simple_char);
		println!("HTML код: {}", simple_char.to_html_code());
		println!("Decimal HTML: {}", simple_char.to_decimal_html_code());
		println!("Hex: {:?}", simple_char.as_hex());
		println!("Составной: {}", simple_char.composition.is_composite());

		println!("\n=== Составной символ (Á) ===");
		println!("{:#?}", composite_char);
		println!("HTML код: {}", composite_char.to_html_code());
		println!("Decimal HTML: {}", composite_char.to_decimal_html_code());
		println!("Hex: {:?}", composite_char.as_hex());
		println!("Составной: {}", composite_char.composition.is_composite());

		println!("\n=== Эмодзи (😀) ===");
		println!("{:#?}", emoji_char);
		println!("HTML код: {}", emoji_char.to_html_code());
		println!("Decimal HTML: {}", emoji_char.to_decimal_html_code());
		println!("Hex: {:?}", emoji_char.as_hex());
		println!("Составной: {}", emoji_char.composition.is_composite());
	}
}
