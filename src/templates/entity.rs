pub const ENTITY: &'static str = r#"
pub trait Entity {
    type Ecs: Ecs;

    fn ecs(&self) -> &Self::Ecs;
    fn id(&self) -> EntityId;

{{#each data_components}}
    fn {{name}}(&self) -> Option<&{{type}}> {
        self.ecs().get_{{name}}(self.id())
    }
    fn contains_{{name}}(&self) -> bool {
        self.ecs().contains_{{name}}(self.id())
    }
    {{#if copy}}
    fn copy_{{name}}(&self) -> Option<{{type}}> {
        self.{{name}}().map(|c| *c)
    }
    {{/if}}
{{/each}}
{{#each cell_components}}
    fn bare_{{name}}(&self) -> Option<&RefCell<{{type}}>> {
        self.ecs().bare_get_{{name}}(self.id())
    }
    fn borrow_{{name}}(&self) -> Option<Ref<{{type}}>> {
        self.ecs().borrow_{{name}}(self.id())
    }
    fn borrow_mut_{{name}}(&self) -> Option<RefMut<{{type}}>> {
        self.ecs().borrow_mut_{{name}}(self.id())
    }
    fn contains_{{name}}(&self) -> bool {
        self.ecs().contains_{{name}}(self.id())
    }
    fn {{name}}(&self) -> Option<&RefCell<{{type}}>> {
        self.bare_{{name}}()
    }
{{/each}}
{{#each flag_components}}
    fn contains_{{name}}(&self) -> bool {
        self.ecs().contains_{{name}}(self.id())
    }
    fn {{name}}(&self) -> bool {
        self.contains_{{name}}()
    }
{{/each}}
}
"#;
