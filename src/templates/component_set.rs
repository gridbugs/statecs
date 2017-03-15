pub const COMPONENT_SET: &'static str = r#"

pub struct ComponentSet {
{{#if single_component_bitfield}}
    bitfield: u64,
{{else}}
    bitfield: [u64; {{component_bitfield_size}}],
{{/if}}
}

impl ComponentSet {

    pub fn new() -> Self {
        ComponentSet {
{{#if single_component_bitfield}}
            bitfield: 0,
{{else}}
            bitfield: [0; {{component_bitfield_size}}],
{{/if}}
        }
    }

    pub fn is_empty(&self) -> bool {
{{#if single_component_bitfield}}
        self.bitfield == 0
{{else}}
        for b in &self.bitfield {
            if *b != 0 {
                return false;
            }
        }

        true
{{/if}}
    }

    pub fn clear(&mut self) {
{{#if single_component_bitfield}}
        self.bitfield = 0
{{else}}
        for b in &mut self.bitfield {
            *b = 0;
        }
{{/if}}
    }

{{#each components}}
    pub fn insert_{{name}}(&mut self) {
    {{#if ../single_component_bitfield}}
        self.bitfield |= {{bitfield_bit}};
    {{else}}
        self.bitfield[{{bitfield_idx}}] |= {{bitfield_bit}};
    {{/if}}
    }

    pub fn remove_{{name}}(&mut self) {
    {{#if ../single_component_bitfield}}
        self.bitfield &= !{{bitfield_bit}};
    {{else}}
        self.bitfield[{{bitfield_idx}}] &= !{{bitfield_bit}};
    {{/if}}
    }

    pub fn contains_{{name}}(&self) -> bool {
    {{#if ../single_component_bitfield}}
        self.bitfield & {{bitfield_bit}} != 0
    {{else}}
        self.bitfield[{{bitfield_idx}}] & {{bitfield_bit}} != 0
    {{/if}}
    }
{{/each}}
}

"#;
