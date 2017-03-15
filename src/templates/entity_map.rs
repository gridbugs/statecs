pub const ENTITY_MAP: &'static str = r#"
#[derive(Clone)]
pub struct EntityMap<T> {
    inner: BTreeMap<EntityId, T>,
}

impl<T> EntityMap<T> {
    pub fn new() -> Self {
        EntityMap {
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

    pub fn iter(&self) -> EntityMapIter<T> {
        EntityMapIter::new(self.inner.iter())
    }

    pub fn keys(&self) -> EntityMapKeys<T> {
        EntityMapKeys::new(self.inner.keys())
    }
}

impl<T: Copy> EntityMap<T> {
    pub fn copy_iter(&self) -> EntityMapCopyIter<T> {
        EntityMapCopyIter::new(self.inner.iter())
    }
}

pub struct EntityMapKeys<'a, T: 'a> {
    keys: btree_map::Keys<'a, EntityId, T>,
}

impl<'a, T: 'a> EntityMapKeys<'a, T> {
    fn new(keys: btree_map::Keys<'a, EntityId, T>) -> Self {
        EntityMapKeys {
            keys: keys,
        }
    }
}

impl<'a, T: 'a> Iterator for EntityMapKeys<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.keys.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityMapIter<'a, T: 'a> {
    iter: btree_map::Iter<'a, EntityId, T>,
}

impl<'a, T: 'a> EntityMapIter<'a, T> {
    fn new(iter: btree_map::Iter<'a, EntityId, T>) -> Self {
        EntityMapIter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a> Iterator for EntityMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id_ref, v)| (*id_ref, v))
    }
}

pub struct EntityMapCopyIter<'a, T: 'a + Copy> {
    iter: btree_map::Iter<'a, EntityId, T>,
}

impl<'a, T: 'a + Copy> EntityMapCopyIter<'a, T> {
    fn new(iter: btree_map::Iter<'a, EntityId, T>) -> Self {
        EntityMapCopyIter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a + Copy> Iterator for EntityMapCopyIter<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id_ref, v)| (*id_ref, *v))
    }
}
"#;
