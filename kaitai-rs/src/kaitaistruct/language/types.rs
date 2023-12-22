use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::enums::Enums;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;
use crate::kaitaistruct::language::meta::Meta;
use crate::kaitaistruct::language::params::Params;
use crate::kaitaistruct::language::seq::Seq;
use std::collections::HashMap;

/// Types struct representing a collection of type specifications
pub struct Types {
    // HashMap mapping identifiers to type specifications
    types: HashMap<Identifier, TypeSpec>,
}

// TypeSpec struct representing a type specification
struct TypeSpec {
    // Metadata for the type
    meta: Meta,
    // Parameters for the type
    params: Params,
    // Sequence of attributes for the type
    seq: Seq,
    // Nested types associated with the type
    type_types: Types,
    // Enumerations associated with the type
    type_enums: Enums,
    // TODO: Replace with instance struct
    instances: String,
    // Documentation for the type
    doc: Doc,
    // Reference to external documentation
    doc_ref: DocRef,
}

impl KaitaiProperty for Types {}
