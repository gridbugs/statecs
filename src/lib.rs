use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Write};

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate handlebars;

mod spec;
mod model;
mod templates;
mod render;
mod config;

use spec::EcsSpec;
use model::EcsModel;

pub use config::Config;

pub fn source_changed<P: AsRef<Path>, Q: AsRef<Path>>(in_path: P, out_path: Q) -> bool {

    let out_time = if let Ok(md) = fs::metadata(out_path) {
        md.modified().expect("Failed to get output file modified time")
    } else {
        return true;
    };

    let in_time = fs::metadata(in_path).expect("Missing input file")
        .modified().expect("Failed to get input file modified time");

    in_time > out_time
}

pub fn generate_content<P: AsRef<Path>, Q: AsRef<Path>>(in_path: P, out_path: Q, config: Config) {

    if !source_changed(&in_path, &out_path) {
        return;
    }

    let mut file = File::open(in_path).expect("Failed to open input file");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("Failed to read input file");

    let ecs_spec: EcsSpec = toml::from_str(string.as_ref())
        .expect("Failed to parse input file");

    let ecs_model = EcsModel::from(&ecs_spec);

    let output_string = render::render_content(&ecs_model, config);
    let mut outfile = File::create(out_path).expect("Failed to create output file");
    write!(outfile, "{}", output_string).expect("Failed to write output file");
}

pub fn generate_core<P: AsRef<Path>>(out_path: P, config: Config) {
    let output_string = render::render_core(config);
    let mut outfile = File::create(out_path).expect("Failed to create output file");
    write!(outfile, "{}", output_string).expect("Failed to write output file");
}
