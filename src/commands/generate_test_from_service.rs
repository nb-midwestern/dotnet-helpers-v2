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
    let constructor_info = get_constructor_with_most_params(&class_info.constructors).unwrap();

    let class_name = constructor_info.clone().name;
    let test_class_name = format!("public class {class_name}Test");
    let build_sut = format!("private {class_name} BuildSystemUnderTest()");
    let build_mock = format!("private Mock<{class_name}> BuildMock()");

    let include_transaction = constructor_info
        .parameters
        .iter()
        .any(|i| i.type_value.contains("IUnitOfWork"));

    let transaction_mock = match include_transaction {
        true => "private readonly Mock<IDbContextTransaction> _transaction = new();".to_string(),
        false => "".to_string(),
    };

    let param_mock = constructor_info
        .parameters
        .iter()
        .map(|param| {
            format!(
                "private readonly Mock<{}> _{} = new();",
                param.type_value, param.identifier,
            )
        })
        .collect::<Vec<String>>()
        .join("\n\t");

    let mock_objects = constructor_info
        .parameters
        .iter()
        .enumerate()
        .map(|(index, i)| {
            let transformed_name = format!("_{}.Object", i.identifier);
            if index < constructor_info.parameters.len() - 1 {
                format!("{},", transformed_name)
            } else {
                transformed_name
            }
        })
        .collect::<Vec<String>>()
        .join("\n\t\t");

    let transaction_setup = if include_transaction {
        format!(
            r#"
        _unitOfWork.Setup(uow => uow.BeginTransaction(It.IsAny<CancellationToken>()))
            .ReturnsAsync(_transaction.Object);

        _transaction.Setup(tran => tran.CommitAsync(It.IsAny<CancellationToken>()))
            .Returns(Task.CompletedTask);

        _transaction.Setup(tran => tran.RollbackAsync(It.IsAny<CancellationToken>()))
            .Returns(Task.CompletedTask);
            "#
        )
    } else {
        String::new()
    };

    return format!(
        r#"
{test_class_name}
{{    
    {param_mock}
    {transaction_mock}
    

    {build_sut}
    {{
         return BuildMock().Object;
    }}

    {build_mock}
    {{
        {transaction_setup}
        {mock_objects}
        return mock;
    }}

    [Fact]
    public async Task {class_name}_ShouldCompile()
    {{
        var _sut = BuildSystemUnderTest();
        Assert.NotNull(_sut);

    }}

}}
    "#
    );
}

fn get_constructor_with_most_params(constructors: &[ConstructorInfo]) -> Option<&ConstructorInfo> {
    constructors
        .iter()
        .max_by_key(|constructor| constructor.parameters.len())
}
