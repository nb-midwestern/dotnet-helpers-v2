use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::app::{App, CommandType};
use crate::csharp::find_class_info::find_class_info;
use crate::csharp::find_constructor_info::find_constructor_info;
use crate::csharp::find_methods::find_methods;
use crate::csharp::find_parameters::find_parameters;
use crate::csharp::tree_sitter;
pub fn generate_test_from_service(app: &mut App) {
    let path = PathBuf::from_str(&app.additional_args).expect("Could not read path buff");
    app.set_input_file(path);
    if let Ok(source_code) =
        fs::read_to_string(app.input_file.clone().to_string_lossy().to_string())
    {
        app.status_message = "Successfully read file".to_string();
        let tree = tree_sitter::get_tree(app.input_file.clone());
        let node = tree.root_node();
        let class_info = find_class_info(&node, &source_code);
        let constructor_info = find_constructor_info(&node, &source_code);
        let methods = find_methods(&node, &source_code);

        if let Some(class) = class_info {
            let out = format!(
                r#"
                {class:?}
                {methods:?}
                "#,
            );

            app.status_message = "Successfully read class information".to_string();
            let out_file_name = "test.out.cs";
            let mut output_file =
                File::create(&Path::new(out_file_name.clone())).expect("could not write to file");
            output_file
                .write_all(out.as_bytes())
                .expect("failed writing data");
            app.status_message = format!("Wrote to file {out_file_name}");
        }
    } else {
        app.status_message = "Could not read file".to_string();
        app.current_command = CommandType::GetInputFile;
    }
}

// c# tree https://tree-sitter.github.io/tree-sitter/playground
// using_directive
// --- qualified_name
// ------- qualifier
// ------- name
// file_scoped_namespace_declaration
// namespace_declaration
// class_declaration
// field_declaration
// constructor_declaration
// // block

// constructor_declaration [13, 4] - [29, 5]
// modifier [13, 4] - [13, 10]
// name: identifier [13, 11] - [13, 29]
// parameters: parameter_list [13, 29] - [20, 117]
//   parameter [14, 8] - [14, 17]
//     type: predefined_type [14, 8] - [14, 11]
//     name: identifier [14, 12] - [14, 17]
//   parameter [15, 8] - [15, 50]
//     type: identifier [15, 8] - [15, 29]
//     name: identifier [15
