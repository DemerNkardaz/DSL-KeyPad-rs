use super::TaxonKind;
use std::collections::HashMap;

pub struct Taxonomy {
	pub nodes: HashMap<TaxonKind, TaxonNode>,
}

#[derive(Debug)]
pub struct TaxonNode {
	pub id: TaxonKind,
	pub parents: &'static [TaxonKind],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Taxon {
	pub id: TaxonKind,
}
