use super::{TaxonNode, Taxonomy};
use std::sync::LazyLock;

static TAXONOMY_TREE: LazyLock<Taxonomy> = LazyLock::new(|| {
	#[allow(clippy::enum_glob_use)]
	use super::TaxonKind::*;

	Taxonomy::new(vec![
		TaxonNode { id: Unknown, parents: &[] },
		TaxonNode { id: Roman, parents: &[] },
		TaxonNode { id: Latin, parents: &[] },
		TaxonNode { id: Cyrillic, parents: &[] },
		TaxonNode { id: Hellenic, parents: &[] },
		TaxonNode { id: Numeral, parents: &[Roman, OldHungarian] },
		TaxonNode { id: Ligature, parents: &[Latin, Cyrillic] },
		TaxonNode { id: Digraph, parents: &[Latin, Cyrillic] },
		TaxonNode {
			id: Accented,
			parents: &[Latin, Cyrillic, Ligature, Digraph],
		},
	])
});

#[cfg(test)]
mod taxontreetest {
	use super::super::TaxonKind;
	use super::*;

	#[test]
	fn test_hierarchy() {
		TAXONOMY_TREE.print_hierarchy();
	}

	#[test]
	fn test_descendant() {
		assert!(TAXONOMY_TREE.is_descendant_of(TaxonKind::Accented, TaxonKind::Latin));

		assert!(!TAXONOMY_TREE.is_descendant_of(TaxonKind::Latin, TaxonKind::Accented));
	}
}
