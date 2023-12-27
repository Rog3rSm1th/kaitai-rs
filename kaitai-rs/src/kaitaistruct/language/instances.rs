use crate::kaitaistruct::language::attribute::Attribute;
use crate::kaitaistruct::language::identifier::Identifier;
use std::collections::HashMap;

/// Instances struct representing a collection of instances
#[allow(dead_code)]
pub struct Instances {
    // Hashmap of instances, where each identifier is associated with an attribute
    instances_spec: HashMap<Identifier, Attribute>,
}

impl Instances {}
