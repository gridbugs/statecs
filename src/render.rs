use handlebars::Handlebars;

use model::*;
use config::*;
use templates;

#[derive(Serialize, Debug)]
struct ComponentTemplateData {
    pub name: String,
    pub id: u64,
    pub bitfield_bit: u64,
    pub bitfield_idx: usize,
    pub uppercase_name: String,
}

#[derive(Serialize, Debug)]
struct DataComponentTemplateData {
    pub name: String,
    pub id: u64,
    pub bitfield_bit: u64,
    pub bitfield_idx: usize,
    pub uppercase_name: String,
    pub type_name: String,
    pub copy: bool,
}

#[derive(Serialize, Debug)]
struct CellComponentTemplateData {
    pub name: String,
    pub id: u64,
    pub bitfield_bit: u64,
    pub bitfield_idx: usize,
    pub uppercase_name: String,
    pub type_name: String,
}

#[derive(Serialize, Debug)]
struct FlagComponentTemplateData {
    pub name: String,
    pub id: u64,
    pub bitfield_bit: u64,
    pub bitfield_idx: usize,
    pub uppercase_name: String,
}

#[derive(Serialize, Debug)]
struct TemplateData {
    pub num_components: usize,
    pub component_bitfield_size: usize,
    pub single_component_bitfield: bool,
    pub components: Vec<ComponentTemplateData>,
    pub data_components: Vec<DataComponentTemplateData>,
    pub cell_components: Vec<CellComponentTemplateData>,
    pub flag_components: Vec<FlagComponentTemplateData>,
}

impl TemplateData {
    fn new(model: &EcsModel, config: Config) -> Self {
        let mut data = TemplateData {
            num_components: model.num_components(),
            component_bitfield_size: model.bitfield_size(),
            single_component_bitfield: config.single_component_bitfield && model.bitfield_size() == 1,
            components: Vec::new(),
            data_components: Vec::new(),
            cell_components: Vec::new(),
            flag_components: Vec::new(),
        };

        for c in model.common.iter() {
            data.components.push(ComponentTemplateData {
                name: c.name.clone(),
                id: c.id,
                bitfield_bit: c.bitfield_bit,
                bitfield_idx: c.bitfield_idx,
                uppercase_name: c.uppercase_name(),
            });
        }

        data
    }
}

pub fn full_template() -> String {
    templates::HEADER.to_string() +
        templates::COMPONENT_SET +
        templates::ENTITY_SET +
        templates::ENTITY_MAP
}

pub fn render(model: &EcsModel, config: Config) -> String {

    let data = TemplateData::new(model, config);

    let mut handlebars = Handlebars::new();

    // prevent xml escaping
    handlebars.register_escape_fn(|input| input.to_string());
    handlebars.template_render(&full_template(), &data)
        .expect("Failed to render template")
}
