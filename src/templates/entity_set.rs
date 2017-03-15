pub const ENTITY_SET: &'static str = r#"
#[derive(Clone)]
pub struct EntitySet {
    inner: BTreeSet<EntityId>,
}

impl EntitySet {
    pub fn new() -> Self {
        EntitySet {
            inner: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId) {
        self.inner.insert(entity);
    }

    pub fn remove(&mut self, entity: EntityId) -> bool {
        self.inner.remove(&entity)
    }

    pub fn contains(&self, entity: EntityId) -> bool {
        self.inner.contains(&entity)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn iter(&self) -> EntitySetIter {
        EntitySetIter::new(self.inner.iter())
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

pub struct EntitySetIter<'a> {
    iter: btree_set::Iter<'a, EntityId>,
}

impl<'a> EntitySetIter<'a> {
    fn new(iter: btree_set::Iter<'a, EntityId>) -> Self {
        EntitySetIter {
            iter: iter,
        }
    }
}

impl<'a> Iterator for EntitySetIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id_ref| *id_ref)
    }
}
"#;
