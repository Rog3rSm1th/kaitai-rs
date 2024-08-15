use kaitai_rs::core::expression::evaluate;
use kaitai_rs::core::kaitai_struct::KaitaiStruct;
use kaitai_rs::ks_language::format_description::FormatDescription;
use std::path::Path;

fn main() {
    // Load the format description from the YAML file
    let format_description = FormatDescription::load_from_file(
        Path::new(file!())
            .parent()
            .unwrap()
            .join("./../../kaitai_struct_formats/executable/android_nanoapp_header.ksy")
            .to_str()
            .unwrap(),
    )
    .unwrap();

    // Parse the file
    let mut parser = KaitaiStruct::new(format_description);
    let path = Path::new(file!())
        .parent()
        .unwrap()
        .join("./files/activity.napp_header");
    parser.parse_file(path).unwrap();

    // Print the AST
    parser.ast.print_ast();

    // Evaluate an expression on this AST and print the result
    let mut evaluated_string = "1";
    let evaluated_expression = evaluate(&parser.ast, evaluated_string);
    println!(
        "\nevaluated_expression : \n{} -> {}",
        evaluated_string, evaluated_expression
    );
}
