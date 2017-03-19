pub const SERIALIZABLE_ECS_CTX: &'static str = r#"
#[derive(Clone, Serialize, Deserialize)]
pub struct SerializableEcsCtx {
{{#each data_components}}
    {{name}}: EcsCtxEntityMap<{{type}}>,
{{/each}}

{{#each cell_components}}
    {{name}}: EcsCtxEntityMap<{{type}}>,
{{/each}}

{{#if combine_flag_set}}
    _flags: EcsCtxEntitySet,
{{else}}
    {{#each flag_components}}
    {{name}}: EcsCtxEntitySet,
    {{/each}}
{{/if}}
{{#if component_bookkeeping}}
    _components: EcsCtxEntityMap<ComponentSet>,
{{/if}}

}

impl From<EcsCtx> for SerializableEcsCtx {
    fn from(ecs: EcsCtx) -> Self {

        let EcsCtx {
{{#each data_components}}
            {{name}},
{{/each}}
{{#each cell_components}}
            mut {{name}},
{{/each}}
{{#if combine_flag_set}}
            _flags,
{{else}}
    {{#each flag_components}}
            {{name}},
    {{/each}}
{{/if}}
{{#if component_bookkeeping}}
            _components,
{{/if}}
        } = ecs;

        SerializableEcsCtx {
{{#each data_components}}
            {{name}}: {{name}},
{{/each}}
{{#each cell_components}}
            {{name}}: {
                let mut map = EcsCtxEntityMap::new();
                let keys: Vec<EntityId> = {{name}}.keys().collect();
                for key in keys {
                    if let Some(cell) = {{name}}.remove(key) {
                        map.insert(key, cell.into_inner());
                    }
                }
                map
            },
{{/each}}
{{#if combine_flag_set}}
            _flags: _flags,
{{else}}
    {{#each flag_components}}
            {{name}}: {{name}},
    {{/each}}
{{/if}}
{{#if component_bookkeeping}}
            _components: _components,
{{/if}}
        }
    }
}

impl From<SerializableEcsCtx> for EcsCtx {
    fn from(ecs: SerializableEcsCtx) -> Self {
        let SerializableEcsCtx {
{{#each data_components}}
            {{name}},
{{/each}}
{{#each cell_components}}
            mut {{name}},
{{/each}}
{{#if combine_flag_set}}
            _flags,
{{else}}
    {{#each flag_components}}
            {{name}},
    {{/each}}
{{/if}}
{{#if component_bookkeeping}}
            _components,
{{/if}}
        } = ecs;

        EcsCtx {
{{#each data_components}}
            {{name}}: {{name}},
{{/each}}
{{#each cell_components}}
            {{name}}: {
                let mut map = EcsCtxEntityMap::new();
                let keys: Vec<EntityId> = {{name}}.keys().collect();
                for key in keys {
                    if let Some(data) = {{name}}.remove(key) {
                        map.insert(key, RefCell::new(data));
                    }
                }
                map
            },
{{/each}}
{{#if combine_flag_set}}
            _flags: _flags,
{{else}}
    {{#each flag_components}}
            {{name}}: {{name}},
    {{/each}}
{{/if}}
{{#if component_bookkeeping}}
            _components: _components,
{{/if}}
        }
    }
}
"#;
