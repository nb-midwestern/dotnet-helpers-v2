#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<(String, String)>, // (type, name)
    pub modifiers: Vec<String>,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub modifiers: Vec<String>,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub modifiers: Vec<String>,
    pub attributes: Vec<String>,
    pub generic_parameters: Vec<String>,
    pub constructors: Vec<ConstructorInfo>,
}

#[derive(Debug, Clone)]
pub struct ConstructorInfo {
    pub name: String,
    pub modifiers: Vec<String>,
    pub attributes: Vec<String>,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub type_value: String,
    pub identifier: String,
}
