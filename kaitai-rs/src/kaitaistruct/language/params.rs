use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::enums::Enum;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;

/// Params struct representing a list of params specifications
pub struct Params {
    // List of params spec
    params_spec: Vec<ParamSpec>,
}

impl KaitaiProperty for Params {}

// ParamSpec struct representing a parameter specification
pub struct ParamSpec {
    // Identifier for the parameter
    id: Identifier,
    // TODO: implement type system
    param_type: String,
    // Documentation for the parameter
    doc: Doc,
    // Reference to external documentation
    doc_ref: DocRef,
    // Enumeration associated with the parameter
    enum_type: Enum,
}
