pub const ECS_CTX: &'static str = r#"
pub struct EcsCtx {
{{#each data_components}}
    {{name}}: EntityMap<{{type}}>,
{{/each}}

{{#each cell_components}}
    {{name}}: EntityMap<RefCell<{{type}}>>,
{{/each}}

{{#if combine_flag_set}}
    _flags: EntitySet,
{{else}}
    {{#each flag_components}}
    {{name}}: EntitySet,
    {{/each}}
{{/if}}

    _components: EntityMap<ComponentSet>,
}

impl EcsCtx {
    pub fn new() -> Self {
        EcsCtx {
{{#each data_components}}
            {{name}}: EntityMap::new(),
{{/each}}

{{#each cell_components}}
            {{name}}: EntityMap::new(),
{{/each}}

{{#if combine_flag_set}}
            _flags: EntitySet::new(),
{{else}}
    {{#each flag_components}}
            {{name}}: EntitySet::new(),
    {{/each}}
{{/if}}

            _components: EntityMap::new(),

        }
    }
}
"#;
