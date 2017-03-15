#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub single_component_bitfield: bool,
    pub combine_flag_set: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            single_component_bitfield: true,
            combine_flag_set: true,
        }
    }
}
