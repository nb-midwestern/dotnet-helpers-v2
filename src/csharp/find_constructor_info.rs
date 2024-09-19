use super::types::{ConstructorInfo, Parameter};

pub fn find_constructor_info(node: &tree_sitter::Node, source_code: &str) -> ConstructorInfo {
    let mut constructor_info = ConstructorInfo {
        name: String::new(),
        modifiers: Vec::new(),
        attributes: Vec::new(),
        parameters: Vec::new(),
    };

    // Extract attributes and modifiers
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "attribute_list" => {
                let attr_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                constructor_info.attributes.push(attr_text);
            }
            "modifier" => {
                let mod_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                constructor_info.modifiers.push(mod_text);
            }
            _ => {}
        }
    }

    // Extract name
    if let Some(name_node) = node.child_by_field_name("name") {
        constructor_info.name = name_node
            .utf8_text(source_code.as_bytes())
            .unwrap()
            .to_string();
    }

    if let Some(parameter_list_node) = node.child_by_field_name("parameters") {
        let mut params_cursor = parameter_list_node.walk();
        for param_node in parameter_list_node.named_children(&mut params_cursor) {
            if param_node.kind() == "parameter" {
                let mut param_type = String::new();
                let mut param_name = String::new();

                let mut param_cursor = param_node.walk();
                let mut type_found = false; // Flag to track if the type was found first

                for child in param_node.named_children(&mut param_cursor) {
                    match child.kind() {
                        // Check for possible type nodes
                        "predefined_type" | "generic_name" | "identifier" if !type_found => {
                            if let Ok(type_text) = child.utf8_text(source_code.as_bytes()) {
                                param_type = type_text.to_string();
                                type_found = true; // Mark that we've identified the type
                            }
                        }
                        // Check for parameter name nodes
                        "identifier" => {
                            if let Ok(name_text) = child.utf8_text(source_code.as_bytes()) {
                                // If type was already found, this identifier is likely the name
                                if type_found {
                                    param_name = name_text.to_string();
                                } else {
                                    // If type is not found, assume this is the type
                                    param_type = name_text.to_string();
                                    type_found = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }

                // Create a Parameter struct instead of a formatted string
                let param = Parameter {
                    type_value: param_type,
                    identifier: param_name,
                };
                constructor_info.parameters.push(param);
            }
        }
    }

    constructor_info
}
