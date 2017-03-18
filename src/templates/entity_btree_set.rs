pub const ENTITY_BTREE_SET: &'static str = r#"
#[derive(Clone)]
pub struct EntityBTreeSet(BTreeSet<EntityId>);

impl EntityBTreeSet {
    pub fn new() -> Self {
        EntityBTreeSet(BTreeSet::new())
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

    pub fn iter(&self) -> EntityBTreeSetIter {
        EntityBTreeSetIter(self.0.iter())
    }

{{#if combine_flag_set}}
    pub fn range<R>(&self, range: R) -> EntityBTreeSetRange
    where R: RangeArgument<EntityId> {
        EntityBTreeSetRange(self.0.range(range))
    }
{{/if}}

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn append(&mut self, other: &mut EntityBTreeSet) {
        self.0.append(&mut other.0);
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
