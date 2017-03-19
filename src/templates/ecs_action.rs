pub const ECS_ACTION: &'static str = r#"

struct EcsActionDeletions {
{{#each components}}
    lookup_{{name}}: EcsActionEntitySet,
    apply_{{name}}: Vec<EntityId>,
{{/each}}
    _empty: bool,
}

impl EcsActionDeletions {
    fn new() -> Self {
        EcsActionDeletions {
{{#each components}}
            lookup_{{name}}: EcsActionEntitySet::new(),
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
    fn delete_{{name}}(&mut self, id: EntityId) -> bool {
        self._empty = false;
        self.apply_{{name}}.push(id);
        self.lookup_{{name}}.insert(id)
    }
    fn iter_{{name}}(&self) -> DeletionIdIter {
        DeletionIdIter(self.apply_{{name}}.iter())
    }
    fn will_delete_{{name}}(&self, id: EntityId) -> bool {
        self.lookup_{{name}}.contains(id)
    }
{{/each}}

    fn entity_delete(&mut self, entity: EntityRef) {
{{#if unchecked_entity_delete}}
    {{#each components}}
        self.delete_{{name}}(entity.id());
    {{/each}}
{{else}}
    {{#if component_bookkeeping}}
        if let Some(component_set) = entity.ecs._components.get(entity.id) {
            for component_id in component_set.iter() {
                match component_id {
        {{#each components}}
                    {{id}} => { self.delete_{{name}}(entity.id); }
        {{/each}}
                    other => { panic!("No such component: {}", other); }
                }
            }
        }
    {{else}}
        {{#each components}}
        if entity.contains_{{name}}() {
            self.delete_{{name}}(entity.id());
        }
        {{/each}}
    {{/if}}
{{/if}}
    }
}

pub struct DeletionIdIter<'a>(slice::Iter<'a, EntityId>);
impl<'a> Iterator for DeletionIdIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| *r)
    }
}

struct EcsActionSwaps {
{{#each components}}
    lookup_{{name}}: EcsActionEntityMap<EntityId>,
    apply_{{name}}: Vec<(EntityId, EntityId)>,
{{/each}}
    _empty: bool,
}

impl EcsActionSwaps {
    fn new() -> Self {
        EcsActionSwaps {
{{#each components}}
            lookup_{{name}}: EcsActionEntityMap::new(),
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
    fn will_swap_{{name}}(&self, id: EntityId) -> Option<EntityId> {
        self.lookup_{{name}}.get(id).map(|r| *r)
    }
{{/each}}
{{#each data_components}}
    fn inner_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapIter<'a, 'b, {{type}}> {
        SwapIter {
            iter: self.lookup_{{name}}.iter(),
            data: &ecs.{{name}},
        }
    }
    fn positive_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveIter<'a, 'b, {{type}}> {
        SwapPositiveIter(self.inner_iter_{{name}}(ecs))
    }
    {{#if copy}}
    fn positive_copy_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveCopyIter<'a, 'b, {{type}}> {
        SwapPositiveCopyIter(self.positive_iter_{{name}}(ecs))
    }
    {{/if}}
    fn positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveIdIter<'a, 'b, {{type}}> {
        SwapPositiveIdIter(self.inner_iter_{{name}}(ecs))
    }
    fn negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapNegativeIdIter<'a, 'b, {{type}}> {
        SwapNegativeIdIter(self.inner_iter_{{name}}(ecs))
    }
{{/each}}
{{#each cell_components}}
    fn inner_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapIter<'a, 'b, RefCell<{{type}}>> {
        SwapIter {
            iter: self.lookup_{{name}}.iter(),
            data: &ecs.{{name}},
        }
    }
    fn positive_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveIter<'a, 'b, RefCell<{{type}}>> {
        SwapPositiveIter(self.inner_iter_{{name}}(ecs))
    }
    fn positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveIdIter<'a, 'b, RefCell<{{type}}>> {
        SwapPositiveIdIter(self.inner_iter_{{name}}(ecs))
    }
    fn negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapNegativeIdIter<'a, 'b, RefCell<{{type}}>> {
        SwapNegativeIdIter(self.inner_iter_{{name}}(ecs))
    }
{{/each}}
{{#each flag_components}}
    fn inner_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapFlagIdIter<'a, 'b> {
        SwapFlagIdIter {
            iter: self.lookup_{{name}}.iter(),
    {{#if ../combine_flag_set}}
            data: &ecs._flags,
            mask: {{mask}},
    {{else}}
            data: &ecs.{{name}},
    {{/if}}
        }
    }
    fn positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveFlagIdIter<'a, 'b> {
        SwapPositiveFlagIdIter(self.inner_id_iter_{{name}}(ecs))
    }
    fn negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapNegativeFlagIdIter<'a, 'b> {
        SwapNegativeFlagIdIter(self.inner_id_iter_{{name}}(ecs))
    }
{{/each}}
}

struct SwapFlagIdIter<'a, 'b> {
    iter: EcsActionEntityMapIter<'a, EntityId>,
    data: &'b EcsCtxEntitySet,
{{#if combine_flag_set}}
    mask: u64,
{{/if}}
}

pub struct SwapPositiveFlagIdIter<'a, 'b>(SwapFlagIdIter<'a, 'b>);
impl<'a, 'b> Iterator for SwapPositiveFlagIdIter<'a, 'b> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, source)) = self.0.iter.next() {
            {{#if combine_flag_set}}
            let key = *source | self.0.mask;
            {{else}}
            let key = *source;
            {{/if}}
            if self.0.data.contains(key) {
                return Some(id);
            }
        }

        None
    }
}

pub struct SwapNegativeFlagIdIter<'a, 'b>(SwapFlagIdIter<'a, 'b>);
impl<'a, 'b> Iterator for SwapNegativeFlagIdIter<'a, 'b> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, source)) = self.0.iter.next() {
            {{#if combine_flag_set}}
            let key = *source | self.0.mask;
            {{else}}
            let key = *source;
            {{/if}}
            if !self.0.data.contains(key) {
                return Some(id);
            }
        }

        None
    }
}

struct SwapIter<'a, 'b, T: 'b> {
    iter: EcsActionEntityMapIter<'a, EntityId>,
    data: &'b EcsCtxEntityMap<T>,
}

pub struct SwapPositiveIter<'a, 'b, T: 'b>(SwapIter<'a, 'b, T>);
impl<'a, 'b, T: 'b> Iterator for SwapPositiveIter<'a, 'b, T> {
    type Item = (EntityId, &'b T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, source)) = self.0.iter.next() {
            if let Some(data) = self.0.data.get(*source) {
                return Some((id, data));
            }
        }

        None
    }
}

pub struct SwapPositiveCopyIter<'a, 'b, T: 'b + Copy>(SwapPositiveIter<'a, 'b, T>);
impl<'a, 'b, T: 'b + Copy> Iterator for SwapPositiveCopyIter<'a, 'b, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id, data)| (id, *data))
    }
}

pub struct SwapPositiveIdIter<'a, 'b, T: 'b>(SwapIter<'a, 'b, T>);
impl<'a, 'b, T: 'b> Iterator for SwapPositiveIdIter<'a, 'b, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, source)) = self.0.iter.next() {
            if self.0.data.contains_key(*source) {
                return Some(id);
            }
        }

        None
    }
}

pub struct SwapNegativeIdIter<'a, 'b, T: 'b>(SwapIter<'a, 'b, T>);
impl<'a, 'b, T: 'b> Iterator for SwapNegativeIdIter<'a, 'b, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, source)) = self.0.iter.next() {
            if !self.0.data.contains_key(*source) {
                return Some(id);
            }
        }

        None
    }
}

pub struct PositiveIter<'a: 'b, 'b, T: 'a + 'b> {
    insertions: EcsCtxEntityMapIter<'a, T>,
    swaps: SwapPositiveIter<'a, 'b, T>,
}
impl<'a: 'b, 'b, T: 'a + 'b> Iterator for PositiveIter<'a, 'b, T> {
    type Item = (EntityId, &'b T);
    fn next(&mut self) -> Option<Self::Item> {
        self.insertions.next().or_else(|| self.swaps.next())
    }
}

pub struct PositiveIdIter<'a: 'b, 'b, T: 'a + 'b> {
    insertions: EcsCtxEntityMapKeys<'a, T>,
    swaps: SwapPositiveIdIter<'a, 'b, T>,
}
impl<'a: 'b, 'b, T: 'a + 'b> Iterator for PositiveIdIter<'a, 'b, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.insertions.next().or_else(|| self.swaps.next())
    }
}

pub struct PositiveCopyIter<'a: 'b, 'b, T: 'a + 'b + Copy> {
    insertions: EcsCtxEntityMapCopyIter<'a, T>,
    swaps: SwapPositiveCopyIter<'a, 'b, T>,
}
impl<'a: 'b, 'b, T: 'a + 'b + Copy> Iterator for PositiveCopyIter<'a, 'b, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.insertions.next().or_else(|| self.swaps.next())
    }
}

pub struct PositiveFlagIdIter<'a: 'b, 'b> {
    insertions: FlagIdIter<'a>,
    swaps: SwapPositiveFlagIdIter<'a, 'b>,
}
impl<'a: 'b, 'b> Iterator for PositiveFlagIdIter<'a, 'b> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.insertions.next().or_else(|| self.swaps.next())
    }
}

pub struct NegativeIdIter<'a: 'b, 'b, T: 'a + 'b> {
    deletions: DeletionIdIter<'a>,
    swaps: SwapNegativeIdIter<'a, 'b, T>,
}
impl<'a: 'b, 'b, T: 'a + 'b> Iterator for NegativeIdIter<'a, 'b, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.deletions.next().or_else(|| self.swaps.next())
    }
}

pub struct NegativeFlagIdIter<'a: 'b, 'b> {
    deletions: DeletionIdIter<'a>,
    swaps: SwapNegativeFlagIdIter<'a, 'b>,
}
impl<'a: 'b, 'b> Iterator for NegativeFlagIdIter<'a, 'b> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.deletions.next().or_else(|| self.swaps.next())
    }
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

    pub fn entity_delete(&mut self, entity: EntityRef) {
        self.deletions.entity_delete(entity);
    }

    pub fn post<'a, 'b>(&'b self, ctx: &'a EcsCtx) -> EcsPostAction<'a, 'b> {
        ctx.post(self)
    }

    pub fn post_entity<'a, 'b>(&'b self, ctx: &'a EcsCtx, id: EntityId) -> EntityRefPostAction<'a, 'b> {
        ctx.post(self).entity(id)
    }

{{#each components}}
    pub fn delete_{{name}}(&mut self, id: EntityId) -> bool {
        self.deletions.delete_{{name}}(id)
    }
    pub fn swap_{{name}}(&mut self, id_a: EntityId, id_b: EntityId) {
        self.swaps.swap_{{name}}(id_a, id_b);
    }
    pub fn deletion_iter_{{name}}(&self) -> DeletionIdIter {
        self.deletions.iter_{{name}}()
    }
    pub fn will_delete_{{name}}(&self, id: EntityId) -> bool {
        self.deletions.will_delete_{{name}}(id)
    }
    pub fn will_swap_{{name}}(&self, id: EntityId) -> Option<EntityId> {
        self.swaps.will_swap_{{name}}(id)
    }
{{/each}}

{{#each data_components}}
    pub fn id_iter_{{name}}(&self) -> EcsCtxEntityMapKeys<{{type}}> {
        self.insertions.id_iter_{{name}}()
    }
    pub fn iter_{{name}}(&self) -> EcsCtxEntityMapIter<{{type}}> {
        self.insertions.iter_{{name}}()
    }
    {{#if copy}}
    pub fn copy_iter_{{name}}(&self) -> EcsCtxEntityMapCopyIter<{{type}}> {
        self.insertions.copy_iter_{{name}}()
    }
    {{/if}}
    pub fn swap_positive_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveIter<'a, 'b, {{type}}> {
        self.swaps.positive_iter_{{name}}(ecs)
    }
    pub fn swap_positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveIdIter<'a, 'b, {{type}}> {
        self.swaps.positive_id_iter_{{name}}(ecs)
    }
    {{#if copy}}
    pub fn swap_positive_copy_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveCopyIter<'a, 'b, {{type}}> {
        self.swaps.positive_copy_iter_{{name}}(ecs)
    }
    {{/if}}
    pub fn swap_negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapNegativeIdIter<'a, 'b, {{type}}> {
        self.swaps.negative_id_iter_{{name}}(ecs)
    }
    pub fn positive_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> PositiveIter<'a, 'b, {{type}}> {
        PositiveIter {
            insertions: self.iter_{{name}}(),
            swaps: self.swap_positive_iter_{{name}}(ecs),
        }
    }
    pub fn positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> PositiveIdIter<'a, 'b, {{type}}> {
        PositiveIdIter {
            insertions: self.id_iter_{{name}}(),
            swaps: self.swap_positive_id_iter_{{name}}(ecs),
        }
    }
    {{#if copy}}
    pub fn positive_copy_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> PositiveCopyIter<'a, 'b, {{type}}> {
        PositiveCopyIter {
            insertions: self.copy_iter_{{name}}(),
            swaps: self.swap_positive_copy_iter_{{name}}(ecs),
        }
    }
    {{/if}}
    pub fn negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> NegativeIdIter<'a, 'b, {{type}}> {
        NegativeIdIter {
            deletions: self.deletion_iter_{{name}}(),
            swaps: self.swap_negative_id_iter_{{name}}(ecs),
        }
    }
    pub fn negative_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> NegativeIdIter<'a, 'b, {{type}}> {
        self.negative_id_iter_{{name}}(ecs)
    }
{{/each}}
{{#each cell_components}}
    pub fn id_iter_{{name}}(&self) -> EcsCtxEntityMapKeys<RefCell<{{type}}>> {
        self.insertions.id_iter_{{name}}()
    }
    pub fn iter_{{name}}(&self) -> EcsCtxEntityMapIter<RefCell<{{type}}>> {
        self.insertions.iter_{{name}}()
    }
    pub fn swap_positive_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveIter<'a, 'b, RefCell<{{type}}>> {
        self.swaps.positive_iter_{{name}}(ecs)
    }
    pub fn swap_positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveIdIter<'a, 'b, RefCell<{{type}}>> {
        self.swaps.positive_id_iter_{{name}}(ecs)
    }
    pub fn swap_negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapNegativeIdIter<'a, 'b, RefCell<{{type}}>> {
        self.swaps.negative_id_iter_{{name}}(ecs)
    }
    pub fn positive_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> PositiveIter<'a, 'b, RefCell<{{type}}>> {
        PositiveIter {
            insertions: self.iter_{{name}}(),
            swaps: self.swap_positive_iter_{{name}}(ecs),
        }
    }
    pub fn positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> PositiveIdIter<'a, 'b, RefCell<{{type}}>> {
        PositiveIdIter {
            insertions: self.id_iter_{{name}}(),
            swaps: self.swap_positive_id_iter_{{name}}(ecs),
        }
    }
    pub fn negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> NegativeIdIter<'a, 'b, RefCell<{{type}}>> {
        NegativeIdIter {
            deletions: self.deletion_iter_{{name}}(),
            swaps: self.swap_negative_id_iter_{{name}}(ecs),
        }
    }
    pub fn negative_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> NegativeIdIter<'a, 'b, RefCell<{{type}}>> {
        self.negative_id_iter_{{name}}(ecs)
    }
{{/each}}
{{#each flag_components}}
    pub fn id_iter_{{name}}(&self) -> FlagIdIter {
        self.insertions.id_iter_{{name}}()
    }
    pub fn swap_positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapPositiveFlagIdIter<'a, 'b> {
        self.swaps.positive_id_iter_{{name}}(ecs)
    }
    pub fn swap_negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> SwapNegativeFlagIdIter<'a, 'b> {
        self.swaps.negative_id_iter_{{name}}(ecs)
    }
    pub fn positive_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> PositiveFlagIdIter<'a, 'b> {
        PositiveFlagIdIter {
            insertions: self.id_iter_{{name}}(),
            swaps: self.swap_positive_id_iter_{{name}}(ecs),
        }
    }
    pub fn negative_id_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> NegativeFlagIdIter<'a, 'b> {
        NegativeFlagIdIter {
            deletions: self.deletion_iter_{{name}}(),
            swaps: self.swap_negative_id_iter_{{name}}(ecs),
        }
    }
    pub fn negative_iter_{{name}}<'a, 'b>(&'a self, ecs: &'b EcsCtx) -> NegativeFlagIdIter<'a, 'b> {
        self.negative_id_iter_{{name}}(ecs)
    }
{{/each}}
}

impl Ecs for EcsAction {
{{#each data_components}}
    fn get_{{name}}(&self, id: EntityId) -> Option<&{{type}}> {
        self.insertions.get_{{name}}(id)
    }
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        self.insertions.contains_{{name}}(id)
    }
{{/each}}
{{#each cell_components}}
    fn bare_get_{{name}}(&self, id: EntityId) -> Option<&RefCell<{{type}}>> {
        self.insertions.bare_get_{{name}}(id)
    }
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        self.insertions.contains_{{name}}(id)
    }
{{/each}}
{{#each flag_components}}
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        self.insertions.contains_{{name}}(id)
    }
{{/each}}
}

impl EcsMut for EcsAction {
{{#each data_components}}
    fn insert_{{name}}(&mut self, id: EntityId, data: {{type}}) -> Option<{{type}}> {
        self.insertions.insert_{{name}}(id, data)
    }
    fn remove_{{name}}(&mut self, id: EntityId) -> Option<{{type}}> {
        self.insertions.remove_{{name}}(id)
    }
    fn get_mut_{{name}}(&mut self, id: EntityId) -> Option<&mut {{type}}> {
        self.insertions.get_mut_{{name}}(id)
    }
{{/each}}
{{#each cell_components}}
    fn bare_insert_{{name}}(&mut self, id: EntityId, data: RefCell<{{type}}>) -> Option<RefCell<{{type}}>> {
        self.insertions.bare_insert_{{name}}(id, data)
    }
    fn bare_remove_{{name}}(&mut self, id: EntityId) -> Option<RefCell<{{type}}>> {
        self.insertions.bare_remove_{{name}}(id)
    }
{{/each}}
{{#each flag_components}}
    fn insert_{{name}}(&mut self, id: EntityId) -> bool {
        self.insertions.insert_{{name}}(id)
    }
    fn remove_{{name}}(&mut self, id: EntityId) -> bool {
        self.insertions.remove_{{name}}(id)
    }
{{/each}}
}
"#;
