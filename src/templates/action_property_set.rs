pub const ACTION_PROPERTY_SET: &'static str = r#"

#[derive(Clone, Serialize, Deserialize)]
pub struct ActionPropertySet {
{{#if single_component_bitfield}}
    bitfield: u64,
{{else}}
    bitfield: [u64; {{action_property_bitfield_size}}],
{{/if}}
}

impl ActionPropertySet {

    pub fn new() -> Self {
        ActionPropertySet {
{{#if single_component_bitfield}}
            bitfield: 0,
{{else}}
            bitfield: [0; {{action_property_bitfield_size}}],
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

{{#each action_properties}}
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
    pub fn iter(&self) -> ActionPropertySetIter {
        ActionPropertySetIter::new(self.bitfield)
    }
}

pub struct ActionPropertySetIter {
{{#if single_component_bitfield}}
    bitfield: u64,
{{else}}
    bitfield: [u64; {{action_property_bitfield_size}}],
    index: usize,
{{/if}}
}

impl ActionPropertySetIter {
{{#if single_component_bitfield}}
    fn new(bitfield: u64) -> Self {
        ActionPropertySetIter {
            bitfield: bitfield,
        }
    }
{{else}}
    fn new(bitfield: [u64; {{action_property_bitfield_size}}]) -> Self {
        ActionPropertySetIter {
            bitfield: bitfield,
            index: 0,
        }
    }
{{/if}}
}

impl Iterator for ActionPropertySetIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
{{#if single_component_bitfield}}
        if self.bitfield == 0 {
            return None;
        }
        let trailing = self.bitfield.trailing_zeros();
        self.bitfield &= !(1 << trailing);
        Some(trailing as u64)
{{else}}
        while self.index < {{action_property_bitfield_size}} && self.bitfield[self.index] == 0 {
            self.index += 1;
        }
        if self.index == {{action_property_bitfield_size}} {
            return None;
        }

        let trailing = self.bitfield[self.index].trailing_zeros();
        self.bitfield[self.index] &= !(1 << trailing);
        Some(self.index as u64 * 64 + trailing as u64)
{{/if}}
    }
}

"#;
