pub const ACTION_PROPERTIES: &'static str = r#"
struct ActionProperties {
{{#if action_property_bookkeeping}}
    _types: ActionPropertySet,
{{/if}}
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
{{#if action_property_bookkeeping}}
            _types: ActionPropertySet::new(),
{{/if}}

{{#each data_action_properties}}
        {{name}}: None,
{{/each}}

{{#each flag_action_properties}}
        {{name}}: false,
{{/each}}
        }
    }

    fn clear(&mut self) {
{{#if action_property_bookkeeping}}
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
        self._types.clear();
{{else}}
    {{#each data_action_properties}}
        self.{{name}} = None;
    {{/each}}
    {{#each flag_action_properties}}
        self.{{name}} = false;
    {{/each}}
{{/if}}
    }


{{#each data_action_properties}}
    fn insert_property_{{name}}(&mut self, data: {{type}}) -> Option<{{type}}> {
    {{#if action_property_bookkeeping}}
        self._types.insert_{{name}}();
    {{/if}}
        mem::replace(&mut self.{{name}}, Some(data))
    }
    fn remove_property_{{name}}(&mut self) -> Option<{{type}}> {
    {{#if action_property_bookkeeping}}
        self._types.remove_{{name}}();
    {{/if}}
        self.{{name}}.take()
    }
    fn get_property_{{name}}(&self) -> Option<&{{type}}> {
        self.{{name}}.as_ref()
    }
    fn contains_property_{{name}}(&self) -> bool {
        self.{{name}}.is_some()
    }
    {{#if copy}}
    fn get_property_copy_{{name}}(&self) -> Option<{{type}}> {
        self.{{name}}
    }
    {{/if}}
{{/each}}

{{#each flag_action_properties}}
    fn insert_property_{{name}}(&mut self) -> bool {
    {{#if action_property_bookkeeping}}
        self._types.insert_{{name}}();
    {{/if}}

        // return true iff this wasn't already set to true
        !mem::replace(&mut self.{{name}}, true)
    }

    fn remove_property_{{name}}(&mut self) -> bool {
    {{#if action_property_bookkeeping}}
        self._types.remove_{{name}}();
    {{/if}}
        // return true iff this was set to true
        mem::replace(&mut self.{{name}}, false)
    }

    fn contains_property_{{name}}(&self) -> bool {
        self.{{name}}
    }
{{/each}}
}
"#;
