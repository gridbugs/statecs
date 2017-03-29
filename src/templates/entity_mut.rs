pub const ENTITY_MUT: &'static str = r#"

pub trait EntityMut: Entity {
    type EcsMut: EcsMut;
    fn ecs_mut(&mut self) -> &mut Self::EcsMut;

{{#each data_components}}
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
{{/each}}
{{#each cell_components}}
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
