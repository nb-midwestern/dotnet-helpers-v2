use std::{fs, path::PathBuf};

use tree_sitter::{Language, Node, Parser, Tree};

pub fn get_tree(path: PathBuf) -> Tree {
    let source_code = fs::read_to_string(path.to_string_lossy().to_string()).unwrap();

    let mut parser = Parser::new();
    let language = Language::new(tree_sitter_c_sharp::LANGUAGE);
    parser
        .set_language(&language)
        .expect("Error loading C# grammar");

    let tree = parser.parse(&source_code, None).unwrap();

    tree
}
