pub const HEADER_CORE: &'static str = r#"// Automatically generated. Do not edit.
use std::collections::{BTreeSet, btree_set};
use std::collections::{BTreeMap, btree_map};
use std::collections::{hash_set, hash_map};

{{#if fnv_hasher}}
use fnv::{FnvHashMap, FnvHashSet};
{{else}}
use std::collections::{HashSet, HashMap};
{{/if}}

{{#if combine_flag_set}}
use collections::Bound;
use collections::range::RangeArgument;
{{/if}}

pub type EntityId = u64;
"#;
