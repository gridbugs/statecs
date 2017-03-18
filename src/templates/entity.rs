pub const ENTITY: &'static str = r#"
pub trait Entity<'a>: 'a + Copy {
    type Ecs: Ecs;

    fn ecs(self) -> &'a Self::Ecs;
    fn id(self) -> EntityId;

{{#each data_components}}
    fn {{name}}(self) -> Option<&'a {{type}}> {
        let ecs = self.ecs();
        ecs.get_{{name}}(self.id())
    }
    {{#if copy}}
    fn copy_{{name}}(self) -> Option<{{type}}> {
        self.{{name}}().map(|c| *c)
    }
    {{/if}}
{{/each}}
}
"#;
