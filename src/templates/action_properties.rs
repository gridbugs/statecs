pub const ACTION_PROPERTIES: &'static str = r#"
struct ActionProperties {

    _types: ActionPropertySet,

{{#each data_action_properties}}
    {{name}}: Option<{{type}}>,
{{/each}}

{{#each flag_action_properties}}
    {{name}}: bool,
{{/each}}
}

impl ActionProperties {
    fn new() -> Self {
        ActionProperties {
            _types: ActionPropertySet::new(),

{{#each data_action_properties}}
        {{name}}: None,
{{/each}}

{{#each flag_action_properties}}
        {{name}}: false,
{{/each}}
        }
    }

    fn clear(&mut self) {
        for id in self._types.iter() {
            match id {
{{#each data_action_properties}}
                {{id}} => { self.{{name}} = None; }
{{/each}}
{{#each flag_action_properties}}
                {{id}} => { self.{{name}} = false; }
{{/each}}
                _ => { panic!("No such action property {}", id) }
            }
        }
    }
}
"#;
