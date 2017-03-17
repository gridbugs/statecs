pub const ENTITY_BTREE_MAP: &'static str = r#"
#[derive(Clone)]
pub struct EntityBTreeMap<T> {
    inner: BTreeMap<EntityId, T>,
}

impl<T> EntityBTreeMap<T> {
    pub fn new() -> Self {
        EntityBTreeMap {
            inner: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId, value: T) -> Option<T> {
        self.inner.insert(entity, value)
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.inner.get(&entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.inner.get_mut(&entity)
    }

    pub fn contains_key(&self, entity: EntityId) -> bool {
        self.inner.contains_key(&entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.inner.remove(&entity)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn entry(&mut self, entity: EntityId) -> btree_map::Entry<EntityId, T> {
        self.inner.entry(entity)
    }

    pub fn iter(&self) -> EntityBTreeMapIter<T> {
        EntityBTreeMapIter::new(self.inner.iter())
    }

    pub fn keys(&self) -> EntityBTreeMapKeys<T> {
        EntityBTreeMapKeys::new(self.inner.keys())
    }

    pub fn append(&mut self, other: &mut EntityBTreeMap<T>) {
        self.inner.append(&mut other.inner)
    }
}

impl<T: Copy> EntityBTreeMap<T> {
    pub fn copy_iter(&self) -> EntityBTreeMapCopyIter<T> {
        EntityBTreeMapCopyIter::new(self.inner.iter())
    }
}

pub struct EntityBTreeMapKeys<'a, T: 'a> {
    keys: btree_map::Keys<'a, EntityId, T>,
}

impl<'a, T: 'a> EntityBTreeMapKeys<'a, T> {
    fn new(keys: btree_map::Keys<'a, EntityId, T>) -> Self {
        EntityBTreeMapKeys {
            keys: keys,
        }
    }
}

impl<'a, T: 'a> Iterator for EntityBTreeMapKeys<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.keys.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityBTreeMapIter<'a, T: 'a> {
    iter: btree_map::Iter<'a, EntityId, T>,
}

impl<'a, T: 'a> EntityBTreeMapIter<'a, T> {
    fn new(iter: btree_map::Iter<'a, EntityId, T>) -> Self {
        EntityBTreeMapIter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a> Iterator for EntityBTreeMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id_ref, v)| (*id_ref, v))
    }
}

pub struct EntityBTreeMapCopyIter<'a, T: 'a + Copy> {
    iter: btree_map::Iter<'a, EntityId, T>,
}

impl<'a, T: 'a + Copy> EntityBTreeMapCopyIter<'a, T> {
    fn new(iter: btree_map::Iter<'a, EntityId, T>) -> Self {
        EntityBTreeMapCopyIter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a + Copy> Iterator for EntityBTreeMapCopyIter<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id_ref, v)| (*id_ref, *v))
    }
}
"#;
