use std::collections::{BTreeMap, BTreeSet};
use spec::*;

#[derive(Debug)]
pub struct CommonComponentModel {
    pub name: String,
    pub id: u64,
    pub bitfield_bit: u64,
    pub bitfield_idx: usize,
}

impl CommonComponentModel {
    fn new(name: String, id: u64) -> Self {
        CommonComponentModel {
            name: name,
            id: id,
            bitfield_bit: 1 << (id % 64),
            bitfield_idx: id as usize / 64,
        }
    }

    pub fn uppercase_name(&self) -> String {
        self.name.to_uppercase()
    }
}

#[derive(Debug)]
pub struct DataComponentModel {
    pub type_name: String,
    pub copy: bool,
}

#[derive(Debug)]
pub struct CellComponentModel {
    pub type_name: String,
}

#[derive(Debug)]
pub struct EcsModel {
    pub common: Vec<CommonComponentModel>,
    pub data: BTreeMap<u64, DataComponentModel>,
    pub cells: BTreeMap<u64, CellComponentModel>,
    pub flags: BTreeSet<u64>,
    pub imports: Vec<String>,
}

impl EcsModel {
    fn new() -> Self {
        EcsModel {
            common: Vec::new(),
            data: BTreeMap::new(),
            cells: BTreeMap::new(),
            flags: BTreeSet::new(),
            imports: Vec::new(),
        }
    }
    pub fn num_components(&self) -> usize { self.common.len() }
    pub fn bitfield_size(&self) -> usize {
        if self.num_components() == 0 {
            1
        } else {
            (self.num_components() - 1) / 64 + 1
        }
    }
}

impl<'a> From<&'a EcsSpec> for EcsModel {
    fn from(spec: &'a EcsSpec) -> Self {
        let mut id = 0;

        let mut model = EcsModel::new();

        for d in spec.data.iter() {
            model.common.push(CommonComponentModel::new(d.name.clone(), id));
            model.data.insert(id, DataComponentModel {
                type_name: d.type_name.clone(),
                copy: d.copy,
            });
            id += 1;
        }

        for c in spec.cells.iter() {
            model.common.push(CommonComponentModel::new(c.name.clone(), id));
            model.cells.insert(id, CellComponentModel {
                type_name: c.type_name.clone(),
            });
            id += 1;
        }

        for f in spec.flags.iter() {
            model.common.push(CommonComponentModel::new(f.name.clone(), id));
            model.flags.insert(id);
            id += 1;
        }

        model.imports = spec.imports.clone();

        model
    }
}
