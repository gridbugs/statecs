pub const ENTITY_BTREE_MAP: &'static str = r#"
#[derive(Clone)]
pub struct EntityBTreeMap<T>(BTreeMap<EntityId, T>);

impl<T> EntityBTreeMap<T> {
    pub fn new() -> Self {
        EntityBTreeMap(BTreeMap::new())
    }

    pub fn insert(&mut self, entity: EntityId, value: T) -> Option<T> {
        self.0.insert(entity, value)
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.0.get(&entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.0.get_mut(&entity)
    }

    pub fn contains_key(&self, entity: EntityId) -> bool {
        self.0.contains_key(&entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.0.remove(&entity)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn entry(&mut self, entity: EntityId) -> btree_map::Entry<EntityId, T> {
        self.0.entry(entity)
    }

    pub fn iter(&self) -> EntityBTreeMapIter<T> {
        EntityBTreeMapIter(self.0.iter())
    }

    pub fn keys(&self) -> EntityBTreeMapKeys<T> {
        EntityBTreeMapKeys(self.0.keys())
    }

    pub fn append(&mut self, other: &mut EntityBTreeMap<T>) {
        self.0.append(&mut other.0)
    }
}

impl<T: Copy> EntityBTreeMap<T> {
    pub fn copy_iter(&self) -> EntityBTreeMapCopyIter<T> {
        EntityBTreeMapCopyIter(self.0.iter())
    }
}

pub struct EntityBTreeMapKeys<'a, T: 'a>(btree_map::Keys<'a, EntityId, T>);

impl<'a, T: 'a> Iterator for EntityBTreeMapKeys<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityBTreeMapIter<'a, T: 'a>(btree_map::Iter<'a, EntityId, T>);

impl<'a, T: 'a> Iterator for EntityBTreeMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id_ref, v)| (*id_ref, v))
    }
}

pub struct EntityBTreeMapCopyIter<'a, T: 'a + Copy>(btree_map::Iter<'a, EntityId, T>);

impl<'a, T: 'a + Copy> Iterator for EntityBTreeMapCopyIter<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id_ref, v)| (*id_ref, *v))
    }
}
"#;
