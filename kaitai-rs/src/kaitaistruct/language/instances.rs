use crate::kaitaistruct::language::attribute::Attribute;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;
use std::collections::HashMap;

/// Instances struct representing a collection of instances
pub struct Instances {
    // Hashmap of instances, where each identifier is associated with an attribute
    instances_spec: HashMap<Identifier, Attribute>,
}

impl KaitaiProperty for Instances {}
