use crate::kaitaistruct::language::attribute::Attribute;
use crate::kaitaistruct::language::identifier::Identifier;
use std::collections::HashMap;

/// Instances struct representing a collection of instances
#[derive(Debug)]
#[allow(dead_code)]
pub struct Instances {
    // Hashmap of instances, where each identifier is associated with an attribute
    instances_spec: HashMap<Identifier, Attribute>,
}

impl Instances {
    /// Constructor function to create a new Instances struct
    pub fn new() -> Self {
        Instances {
            instances_spec: HashMap::new(),
        }
    }

    // Method to add an attribute to the instances
    pub fn add_attribute(&mut self, identifier: Identifier, attribute: Attribute) {
        self.instances_spec.insert(identifier, attribute);
    }
}
