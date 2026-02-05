use std::fmt;

use super::structs::{CharacterComposition, CharacterEntity};

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
