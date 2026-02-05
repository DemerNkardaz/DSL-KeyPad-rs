use strum_macros::{AsRefStr, VariantNames};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VariantNames, AsRefStr)]
pub enum TaxonKind {
	None,
	Unknown,
	// Languages Scripts and General Categories
	Roman,
	Latin,
	Cyrillic,
	Hellenic,

	Shavian,
	Deseret,

	InternationalPhoneticAlphabet,

	GermanicRunes,
	CirthRunes,
	Glagolitic,

	Manichaean,

	AncientNorthArabian,
	AncientSouthArabian,

	OldPersian,
	OldHungarian,

	// Specific Taxons
	Ligature,
	Digraph,
	Accented,
	Numeral,

	// Script Specific
	// Latin
	HellenicDescendent,

	// Germanic Runes
	ElderFuthark,
	AngloSaxonFuthork,
	YoungerFuthark,
	AlmanacRunes,
	LaterYoungerFuthark,
	MedievalRunes,

	// Other
	Alchemical,
	Astronomical,
	Astrological,

	ZhongGuo,
	Dao,
	YiJing,
	TaiXaunJing,
	MaJiang,
}
