pub const ENTITY_HASH_SET: &'static str = r#"
pub struct EntityHashSet(HashSet<EntityId>);

impl EntityHashSet {
    pub fn new() -> Self {
        EntityHashSet(HashSet::new())
    }

    pub fn insert(&mut self, entity: EntityId) -> bool {
        self.0.insert(entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> bool {
        self.0.remove(&entity)
    }

    pub fn contains(&self, entity: EntityId) -> bool {
        self.0.contains(&entity)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn drain(&mut self) -> hash_set::Drain<EntityId> {
        self.0.drain()
    }

    pub fn append(&mut self, other: &mut EntityHashSet) {
        for id in other.drain() {
            self.insert(id);
        }
    }

    pub fn iter(&self) -> EntityHashSetIter {
        EntityHashSetIter(self.0.iter())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct EntityHashSetIter<'a>(hash_set::Iter<'a, EntityId>);

impl<'a> Iterator for EntityHashSetIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}
"#;
