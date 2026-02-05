use super::{TaxonKind, TaxonNode, Taxonomy};
use std::collections::HashSet;

impl Taxonomy {
	#[must_use]
	pub fn new(nodes: Vec<TaxonNode>) -> Self {
		Self {
			nodes: nodes.into_iter().map(|node| (node.id, node)).collect(),
		}
	}

	#[must_use]
	pub fn is_descendant_of(&self, child: TaxonKind, ancestor: TaxonKind) -> bool {
		self.walk_up(child, |id| *id == ancestor)
	}

	fn walk_up<F: Fn(&TaxonKind) -> bool>(&self, start: TaxonKind, predicate: F) -> bool {
		let mut stack = vec![start];
		let mut visited = HashSet::new();

		while let Some(current) = stack.pop() {
			if !visited.insert(current) {
				continue;
			}

			if predicate(&current) {
				return true;
			}

			if let Some(node) = self.nodes.get(&current) {
				stack.extend_from_slice(node.parents);
			}
		}

		false
	}
}
