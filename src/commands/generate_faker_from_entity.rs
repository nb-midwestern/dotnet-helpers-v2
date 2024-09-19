use crate::app::{App, CommandType};
use crate::csharp::find_class_info::find_class_info;
use crate::csharp::types::{ClassInfo, ConstructorInfo};
use crate::csharp::{find_properties, tree_sitter};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn generate_faker_from_entity(app: &mut App) {
    let path = PathBuf::from_str(&app.additional_args).expect("Could not read path buff");
    app.set_input_file(path);
    if let Ok(source_code) =
        fs::read_to_string(app.input_file.clone().to_string_lossy().to_string())
    {
        app.status_message = "Successfully read file".to_string();
        let tree = tree_sitter::get_tree(app.input_file.clone());
        let node = tree.root_node();

        let properties = find_properties::find_properties(&node, &source_code);
        let faker_data = format!("{:?}", properties);

        let out_file_name = "faker.out.cs";
        let mut output_file =
            File::create(&Path::new(out_file_name)).expect("could not write to file");
        output_file
            .write_all(faker_data.as_bytes())
            .expect("failed writing data");
        app.status_message = format!("Wrote to file {out_file_name}");
    }
    app.status_message = "Could not read file".to_string();
}
