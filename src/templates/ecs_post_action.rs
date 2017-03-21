pub const ECS_POST_ACTION: &'static str = r#"
#[derive(Clone, Copy)]
pub struct EcsPostAction<'a, 'b> {
    ecs: &'a EcsCtx,
    action: &'b EcsAction,
}

impl<'a, 'b> EcsPostAction<'a, 'b> {
    pub fn entity(self, id: EntityId) -> EntityRefPostAction<'a, 'b> {
        EntityRefPostAction {
            id: id,
            ecs_post_action: self,
        }
    }
{{#each data_components}}
    pub fn current_get_{{name}}(&self, id: EntityId) -> Option<&{{type}}> {
        self.ecs.get_{{name}}(id)
    }
    {{#if copy}}
    pub fn current_get_copy_{{name}}(&self, id: EntityId) -> Option<{{type}}> {
        self.ecs.get_copy_{{name}}(id)
    }
    {{/if}}
{{/each}}
{{#each cell_components}}
    pub fn current_bare_get_{{name}}(&self, id: EntityId) -> Option<&RefCell<{{type}}>> {
        self.ecs.bare_get_{{name}}(id)
    }
    pub fn current_borrow_{{name}}(&self, id: EntityId) -> Option<Ref<{{type}}>> {
        self.ecs.borrow_{{name}}(id)
    }
    pub fn current_borrow_mut_{{name}}(&self, id: EntityId) -> Option<RefMut<{{type}}>> {
        self.ecs.borrow_mut_{{name}}(id)
    }
{{/each}}
{{#each components}}
    pub fn current_contains_{{name}}(&self, id: EntityId) -> bool {
        self.ecs.contains_{{name}}(id)
    }
{{/each}}
}

impl<'a, 'b> Ecs for EcsPostAction<'a, 'b> {
{{#each data_components}}
    fn get_{{name}}(&self, id: EntityId) -> Option<&{{type}}> {
        if let Some(data) = self.action.get_{{name}}(id) {
            return Some(data);
        }
        if self.action.will_delete_{{name}}(id) {
            return None;
        }
        if let Some(other_id) = self.action.will_swap_{{name}}(id) {
            return self.ecs.get_{{name}}(other_id);
        }
        self.ecs.get_{{name}}(id)
    }
{{/each}}
{{#each cell_components}}
    fn bare_get_{{name}}(&self, id: EntityId) -> Option<&RefCell<{{type}}>> {
        if let Some(data) = self.action.bare_get_{{name}}(id) {
            return Some(data);
        }
        if self.action.will_delete_{{name}}(id) {
            return None;
        }
        if let Some(other_id) = self.action.will_swap_{{name}}(id) {
            return self.ecs.bare_get_{{name}}(other_id);
        }
        self.ecs.bare_get_{{name}}(id)
    }
{{/each}}
{{#each components}}
    fn contains_{{name}}(&self, id: EntityId) -> bool {
        if self.action.contains_{{name}}(id) {
            return true;
        }
        if self.action.will_delete_{{name}}(id) {
            return false;
        }
        if let Some(other_id) = self.action.will_swap_{{name}}(id) {
            return self.ecs.contains_{{name}}(other_id);
        }
        self.ecs.contains_{{name}}(id)
    }
{{/each}}
}
"#;
