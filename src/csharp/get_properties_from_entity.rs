use std::fs;
use tree_sitter::{Language, Parser};

fn main() {
    // Load the C# source code from a file
    let source_code = fs::read_to_string("");

    // Initialize the parser
    let mut parser = Parser::new();

    let language = Language::new(tree_sitter_c_sharp::LANGUAGE);
    parser
        .set_language(&language)
        .expect("Error loading C# grammar");

    // Parse the source code
    let tree = parser.parse(&source_code, None).unwrap();

    // Get the root node
    let root_node = tree.root_node();

    // Recursively traverse the tree and collect properties
    let properties = find_properties(&root_node, &source_code);

    // Print out the properties and their types
    for (prop_type, prop_name) in properties {
        println!("Property: {} {}", prop_type, prop_name);
    }
}

fn find_properties(node: &tree_sitter::Node, source_code: &str) -> Vec<(String, String)> {
    let mut properties = Vec::new();

    if node.kind() == "property_declaration" {
        let mut prop_type = String::new();
        let mut prop_name = String::new();

        let mut cursor = node.walk();
        let mut reached_identifier = false;

        for child in node.named_children(&mut cursor) {
            match child.kind() {
                // Skip modifiers and attributes
                "modifier" | "attribute_list" => {}
                // When we reach the identifier, we have the property name
                "identifier" => {
                    prop_name = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    reached_identifier = true;
                    break; // Exit the loop after finding the identifier
                }
                // Collect all other nodes as part of the type
                _ => {
                    let child_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    if !prop_type.is_empty() {
                        prop_type.push(' ');
                    }
                    prop_type.push_str(&child_text);
                }
            }
        }

        properties.push((prop_type.trim().to_string(), prop_name));
    }

    // Recursively search in child nodes
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        properties.extend(find_properties(&child, source_code));
    }

    properties
}
