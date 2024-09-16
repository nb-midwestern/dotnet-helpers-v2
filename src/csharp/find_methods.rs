use super::{find_parameters, types::MethodInfo};

pub fn find_methods(node: &tree_sitter::Node, source_code: &str) -> Vec<MethodInfo> {
    let mut methods = Vec::new();

    if node.kind() == "method_declaration" {
        let mut method_info = MethodInfo {
            name: String::new(),
            return_type: String::new(),
            parameters: Vec::new(),
            modifiers: Vec::new(),
            attributes: Vec::new(),
        };

        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            match child.kind() {
                "attribute_list" => {
                    // Collect method attributes
                    let attr_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    method_info.attributes.push(attr_text);
                }
                "modifier" => {
                    // Collect method modifiers
                    let mod_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    method_info.modifiers.push(mod_text);
                }
                "return_type" => {
                    // Collect return type
                    method_info.return_type =
                        child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                }
                "identifier" => {
                    // Collect method name
                    method_info.name = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                }
                "parameter_list" => {
                    // Collect parameters
                    method_info.parameters = find_parameters::find_parameters(&child, source_code);
                }
                _ => {}
            }
        }

        methods.push(method_info);
    }

    // Recursively search in child nodes
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        methods.extend(find_methods(&child, source_code));
    }

    methods
}
