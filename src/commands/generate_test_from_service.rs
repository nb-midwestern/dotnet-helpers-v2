use crate::app::{App, CommandType};
use crate::csharp::find_class_info::find_class_info;
use crate::csharp::tree_sitter;
use crate::csharp::types::{ClassInfo, ConstructorInfo};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
pub fn generate_test_from_service(app: &mut App) {
    let path = PathBuf::from_str(&app.additional_args).expect("Could not read path buff");
    app.set_input_file(path);
    if let Ok(source_code) =
        fs::read_to_string(app.input_file.clone().to_string_lossy().to_string())
    {
        app.status_message = "Successfully read file".to_string();
        let tree = tree_sitter::get_tree(app.input_file.clone());
        let node = tree.root_node();
        let class_info = find_class_info(&node, &source_code);

        if let Some(class) = class_info {
            let mock = build_mock_object(class.clone());
            app.status_message = "Successfully read class information".to_string();
            let out_file_name = "test.out.cs";
            let mut output_file =
                File::create(&Path::new(out_file_name)).expect("could not write to file");
            output_file
                .write_all(mock.as_bytes())
                .expect("failed writing data");
            app.status_message = format!("Wrote to file {out_file_name}");
        }
    } else {
        app.status_message = "Could not read file".to_string();
        app.current_command = CommandType::GetInputFile;
    }
}

fn build_mock_object(class_info: ClassInfo) -> String {
    // Attempt to get the constructor with the most parameters
    let constructor_info_option = get_constructor_with_most_params(&class_info.constructors);

    let class_name = class_info.name.clone();
    let test_class_name = format!("public class {class_name}Test");

    // Use AutoFixture with AutoMoqCustomization
    let fixture_field = "private readonly IFixture _fixture;".to_string();

    // Declare the SUT
    let sut_field = format!("private readonly {class_name} _sut;");

    // Determine dependencies from the constructor parameters, if any
    let dependencies = if let Some(constructor_info) = constructor_info_option {
        constructor_info.parameters.clone()
    } else {
        // No constructors found; assume default constructor with no parameters
        Vec::new()
    };

    // Declare mocks for all dependencies
    let mocks_declaration = dependencies
        .iter()
        .map(|param| {
            format!(
                "private readonly Mock<{0}> _{1}Mock;",
                param.type_value,
                to_camel_case(&param.identifier)
            )
        })
        .collect::<Vec<String>>()
        .join("\n    ");

    // Initialize Fixture, Mocks, and SUT in the constructor
    let constructor_body = format!(
        r#"
        _fixture = new Fixture().Customize(new AutoMoqCustomization());
        {mocks_initialization}
        _sut = new {class_name}({dependencies_initialization});
        "#,
        mocks_initialization = dependencies
            .iter()
            .map(|param| {
                format!(
                    "_{0}Mock = new Mock<{1}>();",
                    to_camel_case(&param.identifier),
                    param.type_value
                )
            })
            .collect::<Vec<String>>()
            .join("\n        "),
        dependencies_initialization = dependencies
            .iter()
            .map(|param| format!("_{0}Mock.Object", to_camel_case(&param.identifier)))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // Example test method
    let example_test_method = format!(
        r#"
    [Fact]
    public async Task {class_name}_ExampleTest()
    {{
        // Arrange
        {mocks_setup}

        // Act
        var result = await _sut.MethodUnderTest();

        // Assert
        {assertions}
    }}
    "#,
        mocks_setup = dependencies
            .iter()
            .map(|param| {
                format!(
                    "// Set up _{0}Mock as needed",
                    to_camel_case(&param.identifier)
                )
            })
            .collect::<Vec<String>>()
            .join("\n        "),
        assertions = "// Add your assertions here"
    );

    return format!(
        r#"
{test_class_name}
{{
    {fixture_field}
    {mocks_declaration}
    {sut_field}

    public {class_name}Test()
    {{
        {constructor_body}
    }}
    {example_test_method}
}}
"#,
    );
}
fn get_constructor_with_most_params(constructors: &[ConstructorInfo]) -> Option<&ConstructorInfo> {
    constructors
        .iter()
        .max_by_key(|constructor| constructor.parameters.len())
}

fn should_mock_specific_methods(type_value: &str) -> bool {
    // Return true for types that require specific method setups
    // matches!(type_value, "IProductRepository" | "IVinRepository")
    true
}

fn to_camel_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + chars.as_str(),
    }
}
