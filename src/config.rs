#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub single_component_bitfield: bool,
    pub combine_flag_set: bool,
    pub component_bookkeeping: bool,
    pub action_component_bookkeeping: bool,
    pub action_property_bookkeeping: bool,
    pub unchecked_entity_delete: bool,
    pub ecs_ctx_hash_collections: bool,
    pub ecs_action_hash_collections: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            single_component_bitfield: true,
            combine_flag_set: true,
            component_bookkeeping: false,
            action_component_bookkeeping: true,
            action_property_bookkeeping: false,
            unchecked_entity_delete: true,
            ecs_ctx_hash_collections: false,
            ecs_action_hash_collections: false,
        }
    }
}
