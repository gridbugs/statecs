#[derive(Deserialize, Debug)]
pub struct DataComponentSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default = "return_false")]
    pub copy: bool,
}

#[derive(Deserialize, Debug)]
pub struct CellComponentSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
}

#[derive(Deserialize, Debug)]
pub struct FlagComponentSpec {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct EcsSpec {
    #[serde(default = "Vec::new")]
    pub data: Vec<DataComponentSpec>,
    #[serde(default = "Vec::new")]
    pub cells: Vec<CellComponentSpec>,
    #[serde(default = "Vec::new")]
    pub flags: Vec<FlagComponentSpec>,
    #[serde(default = "Vec::new")]
    pub imports: Vec<String>,
    #[serde(default = "Vec::new")]
    pub action_data: Vec<DataComponentSpec>,
    #[serde(default = "Vec::new")]
    pub action_flags: Vec<FlagComponentSpec>,
}

fn return_false() -> bool { false }
