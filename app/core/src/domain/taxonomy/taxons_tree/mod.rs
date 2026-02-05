use super::{TaxonNode, Taxonomy};
use std::sync::LazyLock;

static TAXONOMY_TREE: LazyLock<Taxonomy> = LazyLock::new(|| {
	#[allow(clippy::enum_glob_use)]
	use super::TaxonKind::*;

	Taxonomy::new(vec![
		TaxonNode { id: ROMAN, name: "Roman", parents: &[] },
		TaxonNode { id: LATIN, name: "Latin", parents: &[] },
		TaxonNode { id: CYRILLIC, name: "Cyrillic", parents: &[] },
		TaxonNode { id: HELLENIC, name: "Hellenic", parents: &[] },
		TaxonNode { id: NUMERAL, name: "Numeral", parents: &[ROMAN] },
		TaxonNode {
			id: LIGATURE,
			name: "Ligature",
			parents: &[LATIN, CYRILLIC],
		},
		TaxonNode {
			id: ACCENTED,
			name: "Accented",
			parents: &[LATIN, LIGATURE],
		},
	])
});
