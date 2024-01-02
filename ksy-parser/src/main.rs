use kaitai_rs::kaitaistruct::parser::parser::KsyParser;

fn main() {
    // Create a new instance of KsyParser
    let mut parser = KsyParser::new();

    // Parse YAML file
    if let Ok(_yaml_value) = parser.parse_yaml("../kaitai_struct_formats/common/bcd.ksy") {
        // Print the content of the doc_instance
        parser.print_struct();
    } else {
        println!("Error parsing YAML file");
    }
}
