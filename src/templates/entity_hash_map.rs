pub const ENTITY_HASH_MAP: &'static str = r#"

{{#if fnv_hasher}}
type InnerHashMap<T> = FnvHashMap<EntityId, T>;
{{else}}
type InnerHashMap<T> = HashMap<EntityId, T>;
{{/if}}

#[derive(Clone, Serialize, Deserialize)]
pub struct EntityHashMap<T>(InnerHashMap<T>);

impl<T> EntityHashMap<T> {
    pub fn new() -> Self {
        EntityHashMap(InnerHashMap::default())
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

    pub fn entry(&mut self, entity: EntityId) -> hash_map::Entry<EntityId, T> {
        self.0.entry(entity)
    }

    pub fn iter(&self) -> EntityHashMapIter<T> {
        EntityHashMapIter(self.0.iter())
    }

    pub fn keys(&self) -> EntityHashMapKeys<T> {
        EntityHashMapKeys(self.0.keys())
    }

    pub fn drain(&mut self) -> hash_map::Drain<EntityId, T> {
        self.0.drain()
    }

    pub fn append(&mut self, other: &mut EntityHashMap<T>) {
        for (id, value) in other.drain() {
            self.insert(id, value);
        }
    }
}

impl<T: Copy> EntityHashMap<T> {
    pub fn copy_iter(&self) -> EntityHashMapCopyIter<T> {
        EntityHashMapCopyIter(self.0.iter())
    }
}

pub struct EntityHashMapKeys<'a, T: 'a>(hash_map::Keys<'a, EntityId, T>);

impl<'a, T: 'a> Iterator for EntityHashMapKeys<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityHashMapCopyIter<'a, T: 'a + Copy>(hash_map::Iter<'a, EntityId, T>);

impl<'a, T: 'a + Copy> Iterator for EntityHashMapCopyIter<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id_ref, v)| (*id_ref, *v))
    }
}

pub struct EntityHashMapIter<'a, T: 'a>(hash_map::Iter<'a, EntityId, T>);

impl<'a, T: 'a> Iterator for EntityHashMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id_ref, v)| (*id_ref, v))
    }
}

"#;
