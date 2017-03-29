pub const ENTITY_REF_POST_ACTION: &'static str = r#"
#[derive(Clone, Copy)]
pub struct EntityRefPostAction<'a, 'b> {
    id: EntityId,
    ecs_post_action: EcsPostAction<'a, 'b>,
}

impl<'a, 'b> Entity for EntityRefPostAction<'a, 'b> {
    type Ecs = EcsPostAction<'a, 'b>;
    fn id(&self) -> EntityId { self.id }
    fn ecs(&self) -> &Self::Ecs { &self.ecs_post_action }
}

impl<'a, 'b> EntityRefPostAction<'a, 'b> {

{{#each data_components}}
    pub fn current_{{name}}(&self) -> Option<&{{type}}> {
        self.ecs_post_action.current_get_{{name}}(self.id())
    }
    {{#if copy}}
    pub fn current_copy_{{name}}(&self) -> Option<{{type}}> {
        self.current_{{name}}().map(|c| *c)
    }
    {{/if}}
{{/each}}
{{#each cell_components}}
    pub fn current_bare_{{name}}(&self) -> Option<&RefCell<{{type}}>> {
        self.ecs_post_action.current_bare_get_{{name}}(self.id())
    }
    pub fn current_borrow_{{name}}(&self) -> Option<Ref<{{type}}>> {
        self.ecs_post_action.current_borrow_{{name}}(self.id())
    }
    pub fn current_borrow_mut_{{name}}(&self) -> Option<RefMut<{{type}}>> {
        self.ecs_post_action.current_borrow_mut_{{name}}(self.id())
    }
    pub fn current_{{name}}(&self) -> Option<&RefCell<{{type}}>> {
        self.current_bare_{{name}}()
    }
{{/each}}
{{#each flag_components}}
    pub fn current_{{name}}(&self) -> bool {
        self.current_contains_{{name}}()
    }
{{/each}}
{{#each components}}
    pub fn current_contains_{{name}}(&self) -> bool {
        self.ecs_post_action.current_contains_{{name}}(self.id())
    }
{{/each}}
}
"#;
