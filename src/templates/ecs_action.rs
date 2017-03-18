pub const ECS_ACTION: &'static str = r#"

struct EcsActionDeletions {
{{#each components}}
    {{name}}: EntityBTreeSet,
{{/each}}
    _empty: bool,
}

impl EcsActionDeletions {
    fn new() -> Self {
        EcsActionDeletions {
{{#each components}}
            {{name}}: EntityBTreeSet::new(),
{{/each}}
            _empty: true,
        }
    }

    fn clear(&mut self) {
{{#each components}}
        self.{{name}}.clear();
{{/each}}
        self._empty = true;
    }

    fn is_empty(&self) -> bool { self._empty }

{{#each components}}
    fn delete_{{name}}(&mut self, id: EntityId) -> bool {
        self._empty = false;
        self.{{name}}.insert(id)
    }
{{/each}}
}

struct EcsActionSwaps {
{{#each components}}
    lookup_{{name}}: EntityBTreeMap<EntityId>,
    apply_{{name}}: Vec<(EntityId, EntityId)>,
{{/each}}
    _empty: bool,
}

impl EcsActionSwaps {
    fn new() -> Self {
        EcsActionSwaps {
{{#each components}}
            lookup_{{name}}: EntityBTreeMap::new(),
            apply_{{name}}: Vec::new(),
{{/each}}
            _empty: true,
        }
    }

    fn clear(&mut self) {
{{#each components}}
        self.lookup_{{name}}.clear();
        self.apply_{{name}}.clear();
{{/each}}
        self._empty = true;
    }

    fn is_empty(&self) -> bool { self._empty }

{{#each components}}
    fn swap_{{name}}(&mut self, id_a: EntityId, id_b: EntityId) {

        let a = self.lookup_{{name}}.get(id_a).map(|r| *r).unwrap_or(id_a);
        let b = self.lookup_{{name}}.get(id_b).map(|r| *r).unwrap_or(id_b);

        self.lookup_{{name}}.insert(id_a, b);
        self.lookup_{{name}}.insert(id_b, a);

        self.apply_{{name}}.push((id_a, id_b));

        self._empty = false;
    }
{{/each}}
}

pub struct EcsAction {
    insertions: EcsCtx,
    deletions: EcsActionDeletions,
    swaps: EcsActionSwaps,
}

impl EcsAction {
    pub fn new() -> Self {
        EcsAction {
            insertions: EcsCtx::new(),
            deletions: EcsActionDeletions::new(),
            swaps: EcsActionSwaps::new(),
        }
    }

    pub fn clear(&mut self) {
        self.insertions.clear();
        self.deletions.clear();
        self.swaps.clear();
    }

    pub fn has_deletions(&self) -> bool { !self.deletions.is_empty() }
    pub fn has_swaps(&self) -> bool { !self.swaps.is_empty() }
    pub fn is_pure(&self) -> bool { self.swaps.is_empty() && self.deletions.is_empty() }

    pub fn entity(&self, id: EntityId) -> EntityRef {
        EntityRef {
            ecs: &self.insertions,
            id: id,
        }
    }

    pub fn entity_mut(&mut self, id: EntityId) -> EntityRefMut {
        EntityRefMut {
            ecs: &mut self.insertions,
            id: id,
        }
    }

{{#each components}}
    pub fn delete_{{name}}(&mut self, id: EntityId) -> bool {
        self.deletions.delete_{{name}}(id)
    }
    pub fn swap_{{name}}(&mut self, id_a: EntityId, id_b: EntityId) {
        self.swaps.swap_{{name}}(id_a, id_b);
    }
{{/each}}

{{#each data_components}}
    pub fn id_iter_{{name}}(&self) -> EntityBTreeMapKeys<{{type}}> {
        self.insertions.id_iter_{{name}}()
    }
    pub fn iter_{{name}}(&self) -> EntityBTreeMapIter<{{type}}> {
        self.insertions.iter_{{name}}()
    }
    {{#if copy}}
    pub fn copy_iter_{{name}}(&self) -> EntityBTreeMapCopyIter<{{type}}> {
        self.insertions.copy_iter_{{name}}()
    }
    {{/if}}
{{/each}}
{{#each cell_components}}
    pub fn id_iter_{{name}}(&self) -> EntityBTreeMapKeys<RefCell<{{type}}>> {
        self.insertions.id_iter_{{name}}()
    }
    pub fn iter_{{name}}(&self) -> EntityBTreeMapIter<RefCell<{{type}}>> {
        self.insertions.iter_{{name}}()
    }
{{/each}}
{{#each flag_components}}
    pub fn id_iter_{{name}}(&self) -> FlagIdIter {
        self.insertions.id_iter_{{name}}()
    }
{{/each}}
}

impl Ecs for EcsAction {
{{#each data_components}}
    fn insert_{{name}}(&mut self, id: EntityId, data: {{type}}) -> Option<{{type}}> {
        self.insertions.insert_{{name}}(id, data)
    }
    fn remove_{{name}}(&mut self, id: EntityId) -> Option<{{type}}> {
        self.insertions.remove_{{name}}(id)
    }
    fn get_{{name}}(&self, id: EntityId) -> Option<&{{type}}> {
        self.insertions.get_{{name}}(id)
    }
    fn get_mut_{{name}}(&mut self, id: EntityId) -> Option<&mut {{type}}> {
        self.insertions.get_mut_{{name}}(id)
    }
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        self.insertions.contains_{{name}}(id)
    }
{{/each}}
{{#each cell_components}}
    fn bare_insert_{{name}}(&mut self, id: EntityId, data: RefCell<{{type}}>) -> Option<RefCell<{{type}}>> {
        self.insertions.bare_insert_{{name}}(id, data)
    }
    fn bare_remove_{{name}}(&mut self, id: EntityId) -> Option<RefCell<{{type}}>> {
        self.insertions.bare_remove_{{name}}(id)
    }
    fn bare_get_{{name}}(&self, id: EntityId) -> Option<&RefCell<{{type}}>> {
        self.insertions.bare_get_{{name}}(id)
    }
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        self.insertions.contains_{{name}}(id)
    }
{{/each}}
{{#each flag_components}}
    fn insert_{{name}}(&mut self, id: EntityId) -> bool {
        self.insertions.insert_{{name}}(id)
    }
    fn remove_{{name}}(&mut self, id: EntityId) -> bool {
        self.insertions.remove_{{name}}(id)
    }
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        self.insertions.contains_{{name}}(id)
    }
{{/each}}
}
"#;
