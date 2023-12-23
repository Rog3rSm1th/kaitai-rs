use kaitai_rs::kaitaistruct::parser::parser::KsyParser;

fn main() {
    // Create a new instance of KsyParser
    let mut parser = KsyParser::new();

    // Parse YAML file
    if let Ok(yaml_value) =
        parser.parse_yaml("../kaitai_struct_formats/archive/android_bootldr_asus.ksy")
    {
        // Process parsed YAML value (you may want to do something with it)
        parser.parse_sections(&yaml_value).unwrap();

        // Print the content of the doc_instance
        parser.print_struct();
    } else {
        println!("Error parsing YAML file");
    }
}
