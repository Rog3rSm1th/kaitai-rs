use crate::core::ast::Node;
use crate::core::ast::AST;
use crate::ks_language::format_description::FormatDescription;
use crate::ks_language::language::kaitai_type::parse_unsigned_integer;
use crate::ks_language::language::kaitai_type::PureType;

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Struct representing a Kaitai struct
#[allow(dead_code)]
pub struct KaitaiStruct {
    data: Vec<u8>,
    pub ast: AST<Vec<u8>>,
    format_description: FormatDescription,
}

impl KaitaiStruct {
    // Create a new instance of `KaitaiStruct` with an empty Vec<u8> for data, a new AST for ast, and the provided FormatDescription
    pub fn new(format_description: FormatDescription) -> Self {
        let data = Vec::<u8>::new();
        let ast = AST::new();

        KaitaiStruct {
            data,
            ast,
            format_description,
        }
    }

    // Get the data from the `KaitaiStruct` instance
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    // Prints the data as a string, even if it contains invalid UTF-8 sequences
    pub fn print_data_as_string_lossy(&self) {
        let data_str: String = self.data.iter().map(|byte| char::from(*byte)).collect();
        println!("{}", data_str);
    }

    /// Parses the data and convert it into an AST
    /// For now it's just a naive implementation that only parses top-level attributes
    /// TODO : Step-by-step improvements to manage more and more features
    fn parse_data(&mut self) {
        let mut root_node = self.ast.get_root().borrow_mut();

        // Keep track of the current offset in the data
        let mut data_offset = 0;

        // Start by parsing top-level elements
        let top_level_seq = &self.format_description.format.seq;
        for attribute in &top_level_seq.attributes {
            // Create the attribute node with ID
            let attribute_id = match &attribute.id {
                Some(id) => id.clone(),
                None => "default_id".to_string(),
            };
            let attribute_node = Node::<Vec<u8>>::new(attribute_id);
            let mut attribute_node_borrowed = attribute_node.borrow_mut();

            // if size-eos is enabled, we read all the bytes till the end of the stream
            if attribute.size_eos {
                let data_clone = self.data.clone();
                attribute_node_borrowed.set_data(data_clone);
            }
            // If seq_type attribute
            else if let Some(seq_type) = &attribute.seq_type {
                match &seq_type.pure_type {
                    PureType::UnsignedInteger(size) => {
                        // Parse the data as an unsigned integer of the given size
                        let value =
                            parse_unsigned_integer(&self.data[data_offset..], *size as usize);
                        attribute_node_borrowed.set_data(value);
                        // Advance the data offset
                        data_offset += *size as usize;
                    }
                    _ => {
                        // TODO: Implement parsing logic for other types
                        todo!();
                    }
                }
            }
            // If contents attribute
            else if let Some(content) = &attribute.contents {
                // Set the data of the attribute node with the contents field data
                attribute_node_borrowed.set_data(content.clone());
            }

            // Create the top-level element node
            let attribute_node_clone = attribute_node.clone();
            root_node.add_child(attribute_node_clone);
        }
    }

    // Parses a file and load its contents into the `KaitaiStruct` instance
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        self.data = data;
        self.ast = AST::new();

        // Parse the file data
        self.parse_data();

        Ok(())
    }
}
