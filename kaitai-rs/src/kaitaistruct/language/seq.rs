use crate::kaitaistruct::language::attribute::Attribute;

/// Seq property struct representing a sequence of attributes
#[allow(dead_code)]
#[derive(Debug)]
pub struct Seq {
    // List of attributes in the sequence
    attributes: Vec<Attribute>,
}

impl Seq {
    // Constructor for Seq with an empty vector
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }
}
