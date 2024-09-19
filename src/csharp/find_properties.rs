pub fn find_properties(node: &tree_sitter::Node, source_code: &str) -> Vec<(String, String)> {
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
