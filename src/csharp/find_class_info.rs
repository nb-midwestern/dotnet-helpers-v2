use super::{find_constructor_info::find_constructor_info, types::ClassInfo};

pub fn find_class_info(node: &tree_sitter::Node, source_code: &str) -> Option<ClassInfo> {
    if node.kind() == "class_declaration" {
        let mut class_info = ClassInfo {
            name: String::new(),
            modifiers: Vec::new(),
            attributes: Vec::new(),
            generic_parameters: Vec::new(),
            constructors: Vec::new(),
        };

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "attribute_list" => {
                    let attr_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    class_info.attributes.push(attr_text);
                }
                "modifier" => {
                    let mod_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    class_info.modifiers.push(mod_text);
                }
                "identifier" => {
                    class_info.name = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                }
                "type_parameter_list" => {
                    let params_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    class_info.generic_parameters.push(params_text);
                }
                "declaration_list" => {
                    // Process class body to find constructors
                    let mut body_cursor = child.walk();
                    for body_child in child.named_children(&mut body_cursor) {
                        if body_child.kind() == "constructor_declaration" {
                            let constructor_info = find_constructor_info(&body_child, source_code);
                            class_info.constructors.push(constructor_info);
                        }
                    }
                }
                _ => {}
            }
        }

        return Some(class_info);
    }

    // Recursively search in child nodes
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if let Some(info) = find_class_info(&child, source_code) {
            return Some(info);
        }
    }

    None
}
