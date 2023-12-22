use crate::kaitaistruct::attribute::Attribute;
use crate::kaitaistruct::kaitai_property::KaitaiProperty;

/// Seq property struct representing a sequence of attributes
pub struct Seq {
    // List of attributes in the sequence
    attributes: Vec<Attribute>,
}

impl KaitaiProperty for Seq {}
