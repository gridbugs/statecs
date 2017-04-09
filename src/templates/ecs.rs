pub const ECS: &'static str = r#"
pub trait Ecs {
{{#each data_components}}
    fn get_{{name}}(&self, id: EntityId) -> Option<&{{type}}>;
    fn get_ptr_{{name}}(&self, id: EntityId) -> Option<*const {{type}}> {
        self.get_{{name}}(id).map(|r| r as *const {{type}})
    }
    fn contains_{{name}}(&self, id: EntityId) -> bool;
    {{#if copy}}
    fn get_copy_{{name}}(&self, id: EntityId) -> Option<{{type}}> {
        self.get_{{name}}(id).map(|c| *c)
    }
    {{/if}}
{{/each}}
{{#each cell_components}}
    fn bare_get_{{name}}(&self, id: EntityId) -> Option<&RefCell<{{type}}>>;
    fn contains_{{name}}(&self, id: EntityId) -> bool;
    fn borrow_{{name}}(&self, id: EntityId) -> Option<Ref<{{type}}>> {
        self.bare_get_{{name}}(id).map(RefCell::borrow)
    }
    fn borrow_mut_{{name}}(&self, id: EntityId) -> Option<RefMut<{{type}}>> {
        self.bare_get_{{name}}(id).map(RefCell::borrow_mut)
    }
{{/each}}
{{#each flag_components}}
    fn contains_{{name}}(&self, id: EntityId) -> bool;
{{/each}}
}
"#;
