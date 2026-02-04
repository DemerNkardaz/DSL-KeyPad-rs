use super::super::entities::character_entity::structs::CharacterEntity;
use std::collections::{HashMap, HashSet};

pub struct CharacterStore {
	entities: HashMap<u32, CharacterEntity>,
	name_index: HashMap<String, u32>,
	tag_index: HashMap<String, Vec<u32>>,
	category_index: HashMap<String, Vec<u32>>,
	word_index: HashMap<String, Vec<u32>>,
}

impl CharacterStore {
	pub fn new(entities: Vec<CharacterEntity>) -> Self {
		let mut store = CharacterStore {
			entities: HashMap::with_capacity(entities.len()),
			name_index: HashMap::with_capacity(entities.len()),
			tag_index: HashMap::new(),
			category_index: HashMap::new(),
			word_index: HashMap::new(),
		};

		for entity in entities {
			store.add(entity);
		}

		store
	}

	pub fn add(&mut self, entity: CharacterEntity) {
		let id = entity.id;

		// 1. Индексация слов для поиска (из имени и описания)
		let words = self.tokenize(&format!("{} {}", entity.unicode_data.name, entity.metadata.description));
		for word in words {
			self.word_index.entry(word).or_default().push(id);
		}

		// 2. Индексация тегов
		for tag in &entity.metadata.tags {
			self.tag_index.entry(tag.to_lowercase()).or_default().push(id);
		}

		// 3. Индексация категории
		self.category_index.entry(entity.metadata.category.to_lowercase()).or_default().push(id);

		// 4. Индекс по точному имени
		self.name_index.insert(entity.unicode_data.name.to_lowercase(), id);

		// 5. Основное хранилище
		self.entities.insert(id, entity);
	}

	pub fn remove(&mut self, id: u32) {
		if let Some(entity) = self.entities.remove(&id) {
			// Очистка name_index
			self.name_index.remove(&entity.unicode_data.name.to_lowercase());

			// Очистка тегов
			for tag in &entity.metadata.tags {
				if let Some(list) = self.tag_index.get_mut(&tag.to_lowercase()) {
					list.retain(|&x| x != id);
				}
			}

			// Очистка категорий
			if let Some(list) = self.category_index.get_mut(&entity.metadata.category.to_lowercase()) {
				list.retain(|&x| x != id);
			}

			// Очистка word_index
			let words = self.tokenize(&format!("{} {}", entity.unicode_data.name, entity.metadata.description));
			for word in words {
				if let Some(list) = self.word_index.get_mut(&word) {
					list.retain(|&x| x != id);
				}
			}
		}
	}

	pub fn get_by_id(&self, id: u32) -> Option<&CharacterEntity> {
		self.entities.get(&id)
	}

	// Вспомогательная функция для разбиения текста на токены (слова)
	fn tokenize(&self, text: &str) -> HashSet<String> {
		text
			.to_lowercase()
			.split(|c: char| !c.is_alphanumeric())
			.filter(|s| s.len() > 1) // Игнорируем слишком короткие связки
			.map(|s| s.to_string())
			.collect()
	}

	// --- Методы поиска ---

	pub fn find_by_name(&self, name: &str) -> Option<&CharacterEntity> {
		self.name_index.get(&name.to_lowercase()).and_then(|id| self.entities.get(id))
	}

	pub fn find_by_tag(&self, tag: &str) -> Vec<&CharacterEntity> {
		self.tag_index.get(&tag.to_lowercase()).map(|ids| ids.iter().filter_map(|id| self.entities.get(id)).collect()).unwrap_or_default()
	}

	pub fn search_by_words(&self, query: &str) -> Vec<&CharacterEntity> {
		let query_words = self.tokenize(query);
		if query_words.is_empty() {
			return vec![];
		}

		// Ищем ID, которые содержат ВСЕ слова из запроса (Intersection)
		let mut sets = query_words.iter().filter_map(|w| self.word_index.get(w));

		let first_set = match sets.next() {
			Some(set) => set.iter().collect::<HashSet<_>>(),
			None => return vec![],
		};

		let result_ids = sets.fold(first_set, |acc, set| {
			let current_set = set.iter().collect::<HashSet<_>>();
			acc.intersection(&current_set).cloned().collect()
		});

		result_ids.iter().filter_map(|&&id| self.entities.get(&id)).collect()
	}
}
