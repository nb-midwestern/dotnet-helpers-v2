pub fn find_namespace(node: &tree_sitter::Node, source_code: &str) -> Option<String> {
    if node.kind() == "file_scoped_namespace_declaration" {
        let mut namespace = String::new();
        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "qualified_name" => {
                    namespace = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
                }
                _ => {}
            }
        }
        return Some(namespace);
    }

    // Recursively search in child nodes
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if let Some(info) = find_namespace(&child, source_code) {
            return Some(info);
        }
    }

    None
}
