use super::types::FieldInfo;

pub fn find_fields(node: &tree_sitter::Node, source_code: &str) -> Vec<FieldInfo> {
    let mut fields = Vec::new();

    if node.kind() == "field_declaration" {
        let mut field_info = FieldInfo {
            name: String::new(),
            field_type: String::new(),
            modifiers: Vec::new(),
            attributes: Vec::new(),
        };

        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            match child.kind() {
                "attribute_list" => {
                    // Collect field attributes
                    let attr_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    field_info.attributes.push(attr_text);
                }
                "modifier" => {
                    // Collect field modifiers
                    let mod_text = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                    field_info.modifiers.push(mod_text);
                }
                "variable_declaration" => {
                    let mut var_cursor = child.walk();
                    for var_child in child.named_children(&mut var_cursor) {
                        match var_child.kind() {
                            "variable_declarator" => {
                                let mut var_decl_cursor = var_child.walk();
                                for var_decl_child in var_child.named_children(&mut var_decl_cursor)
                                {
                                    if var_decl_child.kind() == "identifier" {
                                        field_info.name = var_decl_child
                                            .utf8_text(source_code.as_bytes())
                                            .unwrap()
                                            .to_string();
                                    }
                                }
                            }
                            "type" => {
                                field_info.field_type = var_child
                                    .utf8_text(source_code.as_bytes())
                                    .unwrap()
                                    .to_string();
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        fields.push(field_info);
    }

    // Recursively search in child nodes
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        fields.extend(find_fields(&child, source_code));
    }

    fields
}
