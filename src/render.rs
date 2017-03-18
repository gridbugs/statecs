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
    pub mask: u64,
}

#[derive(Serialize, Debug)]
struct DataComponentTemplateData {
    pub name: String,
    pub id: u64,
    pub bitfield_bit: u64,
    pub bitfield_idx: usize,
    pub uppercase_name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub copy: bool,
    pub mask: u64,
}

#[derive(Serialize, Debug)]
struct CellComponentTemplateData {
    pub name: String,
    pub id: u64,
    pub bitfield_bit: u64,
    pub bitfield_idx: usize,
    pub uppercase_name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub mask: u64,
}

#[derive(Serialize, Debug)]
struct FlagComponentTemplateData {
    pub name: String,
    pub id: u64,
    pub bitfield_bit: u64,
    pub bitfield_idx: usize,
    pub uppercase_name: String,
    pub mask: u64,
}

#[derive(Serialize, Debug)]
struct TemplateData {
    pub num_components: usize,
    pub component_bitfield_size: usize,
    pub single_component_bitfield: bool,
    pub combine_flag_set: bool,
    pub component_bookkeeping: bool,
    pub entity_bits: u32,
    pub component_bits: u32,
    pub entity_mask: u64,
    pub components: Vec<ComponentTemplateData>,
    pub data_components: Vec<DataComponentTemplateData>,
    pub cell_components: Vec<CellComponentTemplateData>,
    pub flag_components: Vec<FlagComponentTemplateData>,
}

impl TemplateData {
    fn new(model: &EcsModel, config: Config) -> Self {

        let num_components = model.num_components();
        let entity_bits = if num_components == 0 {
            64
        } else {
            (num_components - 1).leading_zeros()
        };
        let component_bits = 64 - entity_bits;
        let entity_mask = (1 << entity_bits) - 1;

        let mut data = TemplateData {
            num_components: num_components,
            component_bitfield_size: model.bitfield_size(),
            single_component_bitfield: config.single_component_bitfield && model.bitfield_size() == 1,
            combine_flag_set: config.combine_flag_set,
            component_bookkeeping: config.component_bookkeeping,
            component_bits: component_bits,
            entity_bits: entity_bits,
            entity_mask: entity_mask,
            components: Vec::new(),
            data_components: Vec::new(),
            cell_components: Vec::new(),
            flag_components: Vec::new(),
        };

        for c in model.common.iter() {
            let mask = c.id << entity_bits;

            data.components.push(ComponentTemplateData {
                name: c.name.clone(),
                id: c.id,
                bitfield_bit: c.bitfield_bit,
                bitfield_idx: c.bitfield_idx,
                uppercase_name: c.uppercase_name(),
                mask: mask,
            });

            if let Some(data_component) = model.data.get(&c.id) {
                data.data_components.push(DataComponentTemplateData {
                    name: c.name.clone(),
                    id: c.id,
                    bitfield_bit: c.bitfield_bit,
                    bitfield_idx: c.bitfield_idx,
                    uppercase_name: c.uppercase_name(),
                    type_name: data_component.type_name.clone(),
                    copy: data_component.copy,
                    mask: mask,
                });
            }
            if let Some(cell_component) = model.cells.get(&c.id) {
                data.cell_components.push(CellComponentTemplateData {
                    name: c.name.clone(),
                    id: c.id,
                    bitfield_bit: c.bitfield_bit,
                    bitfield_idx: c.bitfield_idx,
                    uppercase_name: c.uppercase_name(),
                    type_name: cell_component.type_name.clone(),
                    mask: mask,
                });
            }
            if model.flags.contains(&c.id) {
                data.flag_components.push(FlagComponentTemplateData {
                    name: c.name.clone(),
                    id: c.id,
                    bitfield_bit: c.bitfield_bit,
                    bitfield_idx: c.bitfield_idx,
                    uppercase_name: c.uppercase_name(),
                    mask: mask,
                });
            }
        }

        data
    }
}

pub fn full_template() -> String {
    templates::HEADER.to_string() +
        templates::COMPONENT_SET +
        templates::ENTITY_BTREE_SET +
        templates::ENTITY_BTREE_MAP +
        templates::ECS_CTX +
        templates::ECS +
        templates::ENTITY +
        templates::ENTITY_MUT +
        templates::ENTITY_REF +
        templates::ENTITY_REF_MUT
}

pub fn render(model: &EcsModel, config: Config) -> String {

    let data = TemplateData::new(model, config);

    let mut handlebars = Handlebars::new();

    // prevent xml escaping
    handlebars.register_escape_fn(|input| input.to_string());
    handlebars.template_render(&full_template(), &data)
        .expect("Failed to render template")
}
