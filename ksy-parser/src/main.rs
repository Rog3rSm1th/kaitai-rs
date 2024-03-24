use kaitai_rs::kaitaistruct::parser::parser::KsyParser;
use std::env;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path argument is provided
    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }

    // Get the file path from the command-line argument
    let file_path = &args[1];

    // Create a new instance of KsyParser
    let mut parser = KsyParser::new();

    // Parse YAML file
    if let Ok(_yaml_value) = parser.parse_yaml(file_path) {
        // Print the content of the doc_instance
        // parser.print_struct();
    } else {
        println!("Error parsing YAML file");
    }
}
