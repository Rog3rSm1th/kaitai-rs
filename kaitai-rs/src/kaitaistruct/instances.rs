use crate::kaitaistruct::attribute::Attribute;
use crate::kaitaistruct::identifier::Identifier;
use crate::kaitaistruct::kaitai_property::KaitaiProperty;
use crate::kaitaistruct::params::Params;
use std::collections::HashMap;

/// Instances struct representing a collection of instances
pub struct Instances {
    // Hashmap of instances, where each identifier is associated with an attribute
    instances_spec: HashMap<Identifier, Attribute>,
}

impl KaitaiProperty for Instances {}
