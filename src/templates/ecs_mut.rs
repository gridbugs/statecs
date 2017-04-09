pub const ECS_MUT: &'static str = r#"
pub trait EcsMut {
{{#each data_components}}
    fn insert_{{name}}(&mut self, id: EntityId, data: {{type}}) -> Option<{{type}}>;
    fn remove_{{name}}(&mut self, id: EntityId) -> Option<{{type}}>;
    fn get_mut_{{name}}(&mut self, id: EntityId) -> Option<&mut {{type}}>;
    fn get_mut_ptr_{{name}}(&mut self, id: EntityId) -> Option<*mut {{type}}> {
        self.get_mut_{{name}}(id).map(|r| r as *mut {{type}})
    }
    fn swap_{{name}}(&mut self, a_id: EntityId, b_id: EntityId) {
        let maybe_a = self.remove_{{name}}(a_id);
        let maybe_b = self.remove_{{name}}(b_id);
        maybe_a.map(|a| self.insert_{{name}}(b_id, a));
        maybe_b.map(|b| self.insert_{{name}}(a_id, b));
    }
{{/each}}
{{#each cell_components}}
    fn insert_{{name}}(&mut self, id: EntityId, data: {{type}}) -> Option<{{type}}> {
        self.bare_insert_{{name}}(id, RefCell::new(data)).map(RefCell::into_inner)
    }
    fn remove_{{name}}(&mut self, id: EntityId) -> Option<{{type}}> {
        self.bare_remove_{{name}}(id).map(RefCell::into_inner)
    }
    fn bare_insert_{{name}}(&mut self, id: EntityId, data: RefCell<{{type}}>) -> Option<RefCell<{{type}}>>;
    fn bare_remove_{{name}}(&mut self, id: EntityId) -> Option<RefCell<{{type}}>>;
    fn swap_{{name}}(&mut self, a_id: EntityId, b_id: EntityId) {
        let maybe_a = self.bare_remove_{{name}}(a_id);
        let maybe_b = self.bare_remove_{{name}}(b_id);
        maybe_a.map(|a| self.bare_insert_{{name}}(b_id, a));
        maybe_b.map(|b| self.bare_insert_{{name}}(a_id, b));
    }
{{/each}}
{{#each flag_components}}
    fn insert_{{name}}(&mut self, id: EntityId) -> bool;
    fn remove_{{name}}(&mut self, id: EntityId) -> bool;
    fn swap_{{name}}(&mut self, a_id: EntityId, b_id: EntityId) {
        let a = self.remove_{{name}}(a_id);
        let b = self.remove_{{name}}(b_id);
        if a { self.insert_{{name}}(b_id); }
        if b { self.insert_{{name}}(a_id); }
    }
{{/each}}
}
"#;
