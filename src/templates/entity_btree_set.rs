pub const ENTITY_BTREE_SET: &'static str = r#"
#[derive(Clone)]
pub struct EntityBTreeSet {
    inner: BTreeSet<EntityId>,
}

impl EntityBTreeSet {
    pub fn new() -> Self {
        EntityBTreeSet {
            inner: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId) -> bool {
        self.inner.insert(entity)
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

    pub fn iter(&self) -> EntityBTreeSetIter {
        EntityBTreeSetIter(self.inner.iter())
    }

{{#if combine_flag_set}}
    pub fn range(&self, start: Bound<EntityId>, end: Bound<EntityId>) -> EntityBTreeSetRange {
        EntityBTreeSetRange(self.inner.range((start, end)))
    }
{{/if}}

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn append(&mut self, other: &mut EntityBTreeSet) {
        self.inner.append(&mut other.inner);
    }
}

pub struct EntityBTreeSetIter<'a>(btree_set::Iter<'a, EntityId>);

impl<'a> Iterator for EntityBTreeSetIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}

{{#if combine_flag_set}}
pub struct EntityBTreeSetRange<'a>(btree_set::Range<'a, EntityId>);

impl<'a> Iterator for EntityBTreeSetRange<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}
{{/if}}
"#;
