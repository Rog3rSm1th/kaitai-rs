use kaitai_rs::ks_language::format_description::FormatDescription;

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

    // Load the format description from the YAML file
    let format_description = FormatDescription::load_from_file(file_path);

    // Print the parsed format structure
    format_description.unwrap().format.print_struct();
}
