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
    pub data: Vec<DataComponentSpec>,
    pub cells: Vec<CellComponentSpec>,
    pub flags: Vec<FlagComponentSpec>,
    pub imports: Vec<String>,
}

fn return_false() -> bool { false }
