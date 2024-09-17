#[derive(Debug)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<(String, String)>, // (type, name)
    pub modifiers: Vec<String>,
    pub attributes: Vec<String>,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub modifiers: Vec<String>,
    pub attributes: Vec<String>,
}

#[derive(Debug)]
pub struct ClassInfo {
    pub name: String,
    pub modifiers: Vec<String>,
    pub attributes: Vec<String>,
    pub generic_parameters: Vec<String>,
    pub constructors: Vec<ConstructorInfo>,
}

#[derive(Debug)]
pub struct ConstructorInfo {
    pub name: String,
    pub modifiers: Vec<String>,
    pub attributes: Vec<String>,
    pub parameters: Vec<String>,
}
