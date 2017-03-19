pub const ENTITY_MUT: &'static str = r#"
pub trait EntityMut {
    type Ecs: Ecs + EcsMut;

    fn ecs(&self) -> &Self::Ecs;
    fn ecs_mut(&mut self) -> &mut Self::Ecs;
    fn id(&self) -> EntityId;

{{#each data_components}}
    fn {{name}}(&self) -> Option<&{{type}}> {
        self.ecs().get_{{name}}(self.id())
    }
    fn mut_{{name}}(&mut self) -> Option<&mut {{type}}> {
        let id = self.id();
        self.ecs_mut().get_mut_{{name}}(id)
    }
    fn insert_{{name}}(&mut self, data: {{type}}) -> Option<{{type}}> {
        let id = self.id();
        self.ecs_mut().insert_{{name}}(id, data)
    }
    fn remove_{{name}}(&mut self) -> Option<{{type}}> {
        let id = self.id();
        self.ecs_mut().remove_{{name}}(id)
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
    fn insert_{{name}}(&mut self, data: {{type}}) -> Option<{{type}}> {
        let id = self.id();
        self.ecs_mut().insert_{{name}}(id, data)
    }
    fn remove_{{name}}(&mut self) -> Option<{{type}}> {
        let id = self.id();
        self.ecs_mut().remove_{{name}}(id)
    }
    fn bare_insert_{{name}}(&mut self, data: RefCell<{{type}}>) -> Option<RefCell<{{type}}>> {
        let id = self.id();
        self.ecs_mut().bare_insert_{{name}}(id, data)
    }
    fn bare_remove_{{name}}(&mut self) -> Option<RefCell<{{type}}>> {
        let id = self.id();
        self.ecs_mut().bare_remove_{{name}}(id)
    }
{{/each}}
{{#each flag_components}}
    fn contains_{{name}}(&self) -> bool {
        self.ecs().contains_{{name}}(self.id())
    }
    fn {{name}}(&self) -> bool {
        self.contains_{{name}}()
    }
    fn insert_{{name}}(&mut self) -> bool {
        let id = self.id();
        self.ecs_mut().insert_{{name}}(id)
    }
    fn remove_{{name}}(&mut self) -> bool {
        let id = self.id();
        self.ecs_mut().remove_{{name}}(id)
    }
{{/each}}
}
"#;
