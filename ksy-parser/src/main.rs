use kaitai_rs::core::kaitai_struct::KaitaiStruct;
use kaitai_rs::ks_language::format_description::FormatDescription;
use std::env;
use std::path::Path;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments are provided
    if args.len() != 3 {
        println!("Usage: {} <format_description_path> <file_path>", args[0]);
        return;
    }

    // Get the paths from the command-line arguments
    let format_description_path = &args[1];
    let file_path = &args[2];

    // Load the format description from the YAML file
    let format_description =
        FormatDescription::load_from_file(Path::new(format_description_path)).unwrap();

    // Create a new parser with the loaded format description
    let mut parser = KaitaiStruct::new(format_description);

    // Parse the file
    parser.parse_file(Path::new(file_path)).unwrap();

    // Print the AST
    println!("{:#?}", parser.ast);
}
