pub const ECS_CTX: &'static str = r#"

pub struct EcsCtx {
{{#each data_components}}
    {{name}}: EcsCtxEntityMap<{{type}}>,
{{/each}}

{{#each cell_components}}
    {{name}}: EcsCtxEntityMap<RefCell<{{type}}>>,
{{/each}}

{{#if combine_flag_set}}
    _flags: EcsCtxEntitySet,
{{else}}
    {{#each flag_components}}
    {{name}}: EcsCtxEntitySet,
    {{/each}}
{{/if}}
{{#if component_bookkeeping}}
    _components: EcsCtxEntityMap<ComponentSet>,
{{/if}}
}

impl EcsCtx {
    pub fn new() -> Self {
        EcsCtx {
{{#each data_components}}
            {{name}}: EcsCtxEntityMap::new(),
{{/each}}

{{#each cell_components}}
            {{name}}: EcsCtxEntityMap::new(),
{{/each}}

{{#if combine_flag_set}}
            _flags: EcsCtxEntitySet::new(),
{{else}}
    {{#each flag_components}}
            {{name}}: EcsCtxEntitySet::new(),
    {{/each}}
{{/if}}
{{#if component_bookkeeping}}
            _components: EcsCtxEntityMap::new(),
{{/if}}
        }
    }

    pub fn max_entity_id(&self) -> EntityId {
        {{entity_mask}}
    }

    pub fn entity(&self, id: EntityId) -> EntityRef {
        EntityRef {
            id: id,
            ecs: self,
        }
    }

    pub fn entity_mut(&mut self, id: EntityId) -> EntityRefMut {
        EntityRefMut {
            id: id,
            ecs: self,
        }
    }

    pub fn clear(&mut self) {
{{#each data_components}}
        self.{{name}}.clear();
{{/each}}
{{#each cell_components}}
        self.{{name}}.clear();
{{/each}}
{{#each flag_components}}
    {{#if ../combine_flag_set}}
        self._flags.clear();
    {{else}}
        self.{{name}}.clear();
    {{/if}}
{{/each}}
{{#if component_bookkeeping}}
        self._components.clear();
{{/if}}
    }

{{#if component_bookkeeping}}
    {{#each components}}
    fn bookkeeping_insert_{{name}}(&mut self, id: EntityId) {
        self._components.get_mut(id).map(ComponentSet::insert_{{name}});
    }
    fn bookkeeping_remove_{{name}}(&mut self, id: EntityId) {
        self._components.get_mut(id).map(ComponentSet::remove_{{name}});
    }
    {{/each}}
{{/if}}

{{#each data_components}}
    fn inner_insert_{{name}}(&mut self, id: EntityId, data: {{type}}) -> Option<{{type}}> {
        self.{{name}}.insert(id, data)
    }
    fn inner_remove_{{name}}(&mut self, id: EntityId) -> Option<{{type}}> {
        self.{{name}}.remove(id)
    }
    pub fn id_iter_{{name}}(&self) -> EcsCtxEntityMapKeys<{{type}}> {
        self.{{name}}.keys()
    }
    pub fn iter_{{name}}(&self) -> EcsCtxEntityMapIter<{{type}}> {
        self.{{name}}.iter()
    }
    {{#if copy}}
    pub fn copy_iter_{{name}}(&self) -> EcsCtxEntityMapCopyIter<{{type}}> {
        self.{{name}}.copy_iter()
    }
    {{/if}}
{{/each}}

{{#each cell_components}}
    fn inner_bare_insert_{{name}}(&mut self, id: EntityId, data: RefCell<{{type}}>) -> Option<RefCell<{{type}}>> {
        self.{{name}}.insert(id, data)
    }
    fn inner_bare_remove_{{name}}(&mut self, id: EntityId) -> Option<RefCell<{{type}}>> {
        self.{{name}}.remove(id)
    }
    pub fn id_iter_{{name}}(&self) -> EcsCtxEntityMapKeys<RefCell<{{type}}>> {
        self.{{name}}.keys()
    }
    pub fn iter_{{name}}(&self) -> EcsCtxEntityMapIter<RefCell<{{type}}>> {
        self.{{name}}.iter()
    }
{{/each}}

    fn mask_flag_key(key: u64) -> EntityId {
        key & {{../entity_mask}}
    }

{{#each flag_components}}
    {{#if ../combine_flag_set}}
    fn flag_key_{{name}}(id: EntityId) -> u64 {
        id | {{mask}}
    }
    {{/if}}
    fn inner_insert_{{name}}(&mut self, id: EntityId) -> bool {
    {{#if ../combine_flag_set}}
        self._flags.insert(Self::flag_key_{{name}}(id))
    {{else}}
        self.{{name}}.insert(id)
    {{/if}}
    }
    fn inner_remove_{{name}}(&mut self, id: EntityId) -> bool {
    {{#if ../combine_flag_set}}
        self._flags.remove(Self::flag_key_{{name}}(id))
    {{else}}
        self.{{name}}.remove(id)
    {{/if}}
    }

    pub fn id_iter_{{name}}(&self) -> FlagIdIter {
    {{#if ../combine_flag_set}}
        let start = Bound::Included({{mask}});
        let end = Bound::Included({{mask}} | {{../entity_mask}});

        self._flags.range((start, end))
    {{else}}
        self.{{name}}.iter()
    {{/if}}
    }

{{/each}}

    fn commit_swaps(&mut self, swaps: &mut EcsActionSwaps) {
{{#each components}}
        for (id_a, id_b) in swaps.apply_{{name}}.drain(..) {
            self.swap_{{name}}(id_a, id_b);
        }
{{/each}}

        swaps.clear();
    }

    fn commit_deletions(&mut self, deletions: &mut EcsActionDeletions) {
{{#each components}}
        for id in deletions.apply_{{name}}.drain(..) {
            self.remove_{{name}}(id);
        }
{{/each}}

        deletions.clear();
    }

    fn commit_deletions_into(&mut self, deletions: &mut EcsActionDeletions, dest: &mut EcsAction) {
{{#each data_components}}
        for id in deletions.apply_{{name}}.drain(..) {
            self.remove_{{name}}(id).map(|c| dest.insert_{{name}}(id, c));
        }
{{/each}}
{{#each cell_components}}
        for id in deletions.apply_{{name}}.drain(..) {
            self.remove_{{name}}(id).map(|c| dest.insert_{{name}}(id, c));
        }
{{/each}}
{{#each flag_components}}
        for id in deletions.apply_{{name}}.drain(..) {
            if self.remove_{{name}}(id) {
                dest.insert_{{name}}(id);
            }
        }
{{/each}}

        deletions.clear();
    }

    fn commit_insertions(&mut self, insertions: &mut EcsCtx) {
{{#each data_components}}
        self.{{name}}.append(&mut insertions.{{name}});
{{/each}}
{{#each cell_components}}
        self.{{name}}.append(&mut insertions.{{name}});
{{/each}}
{{#if combine_flag_set}}
        self._flags.append(&mut insertions._flags);
{{else}}
    {{#each flag_components}}
        self.{{name}}.append(&mut insertions.{{name}});
    {{/each}}
{{/if}}
    }

    pub fn commit(&mut self, action: &mut EcsAction) {

        if action.has_swaps() {
            self.commit_swaps(&mut action.swaps);
        }

        if action.has_deletions() {
            self.commit_deletions(&mut action.deletions);
        }

        self.commit_insertions(&mut action.insertions);

        action.properties.clear();
    }

    pub fn commit_into(&mut self, action: &mut EcsAction, dest: &mut EcsAction) {

        if action.has_swaps() {
            self.commit_swaps(&mut action.swaps);
        }

        if action.has_deletions() {
            self.commit_deletions_into(&mut action.deletions, dest);
        }

        self.commit_insertions(&mut action.insertions);

        action.properties.clear();
    }

    pub fn post<'a, 'b>(&'a self, action: &'b EcsAction) -> EcsPostAction<'a, 'b> {
        EcsPostAction {
            ecs: self,
            action: action,
        }
    }

    pub fn post_entity<'a, 'b>(&'a self, action: &'b EcsAction, id: EntityId) -> EntityRefPostAction<'a, 'b> {
        self.post(action).entity(id)
    }

    pub fn entity_iter<I: Iterator<Item=EntityId>>(&self, iter: I) -> EntityRefIter<I> {
        EntityRefIter {
            ctx: self,
            iter: iter,
        }
    }
}

impl Ecs for EcsCtx {
{{#each data_components}}
    fn get_{{name}}(&self, id: EntityId) -> Option<&{{type}}> {
        self.{{name}}.get(id)
    }
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        self.{{name}}.contains_key(id)
    }
{{/each}}
{{#each cell_components}}
    fn bare_get_{{name}}(&self, id: EntityId) -> Option<&RefCell<{{type}}>> {
        self.{{name}}.get(id)
    }
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        self.{{name}}.contains_key(id)
    }
{{/each}}
{{#each flag_components}}
    fn contains_{{name}}(&self, id: EntityId) -> bool {
    {{#if ../combine_flag_set}}
        self._flags.contains(Self::flag_key_{{name}}(id))
    {{else}}
        self.{{name}}.contains(id)
    {{/if}}
    }
{{/each}}
}

impl EcsMut for EcsCtx {
{{#each data_components}}
    fn insert_{{name}}(&mut self, id: EntityId, data: {{type}}) -> Option<{{type}}> {
    {{#if ../component_bookkeeping}}
        self.bookkeeping_insert_{{name}}(id);
    {{/if}}
        self.inner_insert_{{name}}(id, data)
    }
    fn remove_{{name}}(&mut self, id: EntityId) -> Option<{{type}}> {
    {{#if ../component_bookkeeping}}
        self.bookkeeping_remove_{{name}}(id);
    {{/if}}
        self.inner_remove_{{name}}(id)
    }
    fn get_mut_{{name}}(&mut self, id: EntityId) -> Option<&mut {{type}}> {
        self.{{name}}.get_mut(id)
    }
{{/each}}
{{#each cell_components}}
    fn bare_insert_{{name}}(&mut self, id: EntityId, data: RefCell<{{type}}>) -> Option<RefCell<{{type}}>> {
    {{#if ../component_bookkeeping}}
        self.bookkeeping_insert_{{name}}(id);
    {{/if}}
        self.inner_bare_insert_{{name}}(id, data)
    }
    fn bare_remove_{{name}}(&mut self, id: EntityId) -> Option<RefCell<{{type}}>> {
    {{#if ../component_bookkeeping}}
        self.bookkeeping_remove_{{name}}(id);
    {{/if}}
        self.inner_bare_remove_{{name}}(id)
    }
{{/each}}
{{#each flag_components}}
    fn insert_{{name}}(&mut self, id: EntityId) -> bool {
    {{#if ../component_bookkeeping}}
        self.bookkeeping_insert_{{name}}(id);
    {{/if}}
        self.inner_insert_{{name}}(id)
    }
    fn remove_{{name}}(&mut self, id: EntityId) -> bool {
    {{#if ../component_bookkeeping}}
        self.bookkeeping_remove_{{name}}(id);
    {{/if}}
        self.inner_remove_{{name}}(id)
    }
{{/each}}
}

pub struct EntityRefIter<'a, I: Iterator<Item=EntityId>> {
    ctx: &'a EcsCtx,
    iter: I,
}

impl<'a, I: Iterator<Item=EntityId>> Iterator for EntityRefIter<'a, I> {
    type Item = EntityRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id| self.ctx.entity(id))
    }
}
"#;
