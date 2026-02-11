use super::structs::CharacterLocalizationParts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterAlternativeDisplay {
	None,
	Sequence(String),
	HTMLNodes(Vec<String>),
	Picture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterLocalizationPart {
	None,
	Composition(CharacterLocalizationParts),
	Single(String),
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
pub enum HotkeysMode {
	Common,
	SecondaryKeys,
	TertiaryKeys,
}
