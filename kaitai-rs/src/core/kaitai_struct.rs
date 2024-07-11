use crate::core::ast::Node;
use crate::core::ast::NodeType;
use crate::core::ast::AST;
use crate::core::expression::evaluate;
use crate::ks_language::format_description::FormatDescription;
use crate::ks_language::language::attribute::Attribute;
use crate::ks_language::language::attribute::Repeat;
use crate::ks_language::language::kaitai_type::PureType;
use crate::ks_language::language::kaitai_type::{parse_strz, parse_unsigned_integer};

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Struct representing a Kaitai struct
#[allow(dead_code)]
pub struct KaitaiStruct {
    data: Vec<u8>,
    pub ast: AST,
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

    // Parses an unsigned integer attribute
    fn parse_unsigned_integer_attribute(
        &self,
        attribute: &Attribute,
        attribute_node: &mut Node,
        data_offset: &mut usize,
    ) {
        let size = match &attribute.seq_type {
            Some(seq_type) => match &seq_type.pure_type {
                PureType::UnsignedInteger(size) => *size,
                _ => return,
            },
            None => return,
        };

        let value = parse_unsigned_integer(&self.data[*data_offset..], size as usize);
        attribute_node.set_data(value);
        attribute_node.set_node_type(NodeType::Integer);
        *data_offset += size as usize;
    }

    // Parses a null-terminated string attribute
    fn parse_stringz_attribute(
        &self,
        attribute: &Attribute,
        attribute_node: &mut Node,
        data_offset: &mut usize,
    ) {
        let size = attribute
            .size
            .as_ref()
            .and_then(|s| s.parse::<usize>().ok());
        let terminator = attribute.terminator.unwrap_or(0);
        let repeat_count = if let Some(repeat) = &attribute.repeat {
            if let Repeat::Expr = repeat {
                let count = evaluate(&self.ast, &attribute.repeat_expr.as_ref().unwrap());
                count as usize
            } else {
                1
            }
        } else {
            1
        };

        for _ in 0..repeat_count {
            let string_data = parse_strz(&self.data[*data_offset..], size, terminator);
            *data_offset += string_data.len();

            if repeat_count > 1 {
                let stringz_node = Node::new(None);
                stringz_node.borrow_mut().set_data(string_data.clone());
                stringz_node.borrow_mut().set_node_type(NodeType::String);
                attribute_node.add_child(stringz_node);
            } else {
                attribute_node.set_data(string_data);
                attribute_node.set_node_type(NodeType::String);
            }
        }
    }

    // Parses a contents attribute
    fn parse_contents_attribute(&self, attribute: &Attribute, attribute_node: &mut Node) {
        if let Some(content) = &attribute.contents {
            attribute_node.set_data(content.clone());
            attribute_node.set_node_type(NodeType::Array);
        }
    }

    // Parses a size attribute
    fn parse_size_attribute(
        &self,
        attribute: &Attribute,
        attribute_node: &mut Node,
        data_offset: &mut usize,
    ) {
        if let Some(size) = &attribute.size {
            if let Ok(size) = size.parse::<usize>() {
                let raw_data = self.data[*data_offset..*data_offset + size].to_vec();
                attribute_node.set_data(raw_data);
                attribute_node.set_node_type(NodeType::Array);
                *data_offset += size;
            }
        }
    }

    // Parses a single attribute
    fn parse_attribute(
        &self,
        attribute: &Attribute,
        attribute_node: &mut Node,
        data_offset: &mut usize,
    ) {
        if attribute.size_eos {
            attribute_node.set_data(self.data.clone());
            attribute_node.set_node_type(NodeType::Array);
        } else if let Some(seq_type) = &attribute.seq_type {
            match &seq_type.pure_type {
                PureType::UnsignedInteger(_) => {
                    self.parse_unsigned_integer_attribute(attribute, attribute_node, data_offset);
                }
                PureType::StringZ => {
                    self.parse_stringz_attribute(attribute, attribute_node, data_offset);
                }
                _ => {
                    // TODO: Implement parsing logic for other types
                    todo!();
                }
            }
        } else if attribute.contents.is_some() {
            self.parse_contents_attribute(attribute, attribute_node);
        } else if attribute.size.is_some() {
            self.parse_size_attribute(attribute, attribute_node, data_offset);
        }
    }

    /// Parses the data and converts it into an AST
    /// For now, it's just a naive implementation that only parses top-level attributes
    /// TODO: Step-by-step improvements to manage more and more features
    fn parse_data(&mut self) {
        let root_node = self.ast.get_root().clone();

        // Keep track of the current offset in the data
        let mut data_offset = 0;

        // Iterate through top-level attributes defined in the format description
        for attribute in &self.format_description.format.seq.attributes {
            // Init the attribute node
            // We use a default ID that is replaced afterwards
            let attribute_id = attribute
                .id
                .clone()
                .unwrap_or_else(|| "default_id".to_string());
            let attribute_node = Node::new(Some(attribute_id.clone()));

            {
                let mut attribute_node_borrowed = attribute_node.borrow_mut();
                self.parse_attribute(attribute, &mut attribute_node_borrowed, &mut data_offset);
            } // Borrow of attribute_node ends here

            // Add the attribute node to the root node outside of the borrow scope
            root_node.borrow_mut().add_child(attribute_node);
        }
    }

    // Parses a file and loads its contents into the `KaitaiStruct` instance
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
