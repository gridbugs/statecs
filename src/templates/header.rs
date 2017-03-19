pub const HEADER: &'static str = r#"// Automatically generated. Do not edit.
#![allow(unused_imports)]
#![allow(dead_code)]

{{#each imports}}
use {{ this }};
{{/each}}

const NUM_COMPONENTS: usize = {{num_components}};

pub type EntityId = u64;
"#;
