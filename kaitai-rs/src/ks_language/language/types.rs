use crate::ks_language::language::doc::Doc;
use crate::ks_language::language::doc_ref::DocRef;
use crate::ks_language::language::enums::Enums;
use crate::ks_language::language::identifier::Identifier;
use crate::ks_language::language::meta::Meta;
use crate::ks_language::language::params::Params;
use crate::ks_language::language::seq::Seq;
use std::collections::HashMap;

/// Types struct representing a collection of type specifications
#[allow(dead_code)]
#[derive(Debug)]
pub struct Types {
    // HashMap mapping identifiers to type specifications
    types: HashMap<Identifier, TypeSpec>,
}

impl Types {
    /// Constructor for creating a new instance of Types
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    /// Adds a TypeSpec to the Types instance.
    pub fn add_typespec(
        &mut self,
        identifier: Identifier,
        typespec_instance: TypeSpec,
    ) -> Result<(), std::io::Error> {
        self.types.insert(identifier, typespec_instance);
        Ok(())
    }
}

// TypeSpec struct representing a type specification
#[allow(dead_code)]
#[derive(Debug)]
pub struct TypeSpec {
    // Metadata for the type
    pub meta: Meta,
    // Parameters for the type
    pub params: Params,
    // Sequence of attributes for the type
    pub seq: Seq,
    // Nested types associated with the type
    pub type_types: Types,
    // Enumerations associated with the type
    pub type_enums: Enums,
    // TODO: Replace with instance struct
    pub instances: String,
    // Documentation for the type
    pub doc: Doc,
    // Reference to external documentation
    pub doc_ref: DocRef,
}

impl TypeSpec {
    pub fn new(
        meta: Meta,
        params: Params,
        seq: Seq,
        type_types: Types,
        type_enums: Enums,
        instances: String,
        doc: Doc,
        doc_ref: DocRef,
    ) -> Self {
        Self {
            meta,
            params,
            seq,
            type_types,
            type_enums,
            instances,
            doc,
            doc_ref,
        }
    }
}
