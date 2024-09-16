pub fn find_parameters(node: &tree_sitter::Node, source_code: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "parameter" {
            let mut param_type = String::new();
            let mut param_name = String::new();

            let mut param_cursor = child.walk();
            for param_child in child.named_children(&mut param_cursor) {
                match param_child.kind() {
                    "identifier" => {
                        param_name = param_child
                            .utf8_text(source_code.as_bytes())
                            .unwrap()
                            .to_string();
                    }
                    _ => {
                        let param_text = param_child
                            .utf8_text(source_code.as_bytes())
                            .unwrap()
                            .to_string();
                        if !param_type.is_empty() {
                            param_type.push(' ');
                        }
                        param_type.push_str(&param_text);
                    }
                }
            }

            params.push((param_type.trim().to_string(), param_name));
        }
    }

    params
}
