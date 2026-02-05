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

impl Taxonomy {
	pub fn print_hierarchy(&self) {
		let roots: Vec<_> = self.nodes.values().filter(|node| node.parents.is_empty()).collect();

		for root in roots {
			self.print_node(root.id, 0, &mut vec![]);
		}
	}

	fn print_node(&self, id: TaxonKind, depth: usize, path: &mut Vec<TaxonKind>) {
		// Проверка на цикл
		if path.contains(&id) {
			return;
		}

		let indent = "  ".repeat(depth);
		println!("{}├─ {}", indent, id.as_ref());

		path.push(id);

		for node in self.nodes.values() {
			if node.parents.contains(&id) {
				self.print_node(node.id, depth + 1, path);
			}
		}

		path.pop();
	}
}
