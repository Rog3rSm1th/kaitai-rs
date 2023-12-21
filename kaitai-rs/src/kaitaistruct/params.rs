use crate::kaitaistruct::attributes::Identifier;
use crate::kaitaistruct::doc::Doc;
use crate::kaitaistruct::doc_ref::DocRef;
use crate::kaitaistruct::enums::Enum;
use crate::kaitaistruct::kaitai_property::KaitaiProperty;

/// Params struct
pub struct Params {
    // List of params spec
    params_spec: Vec<ParamSpec>,
}

impl KaitaiProperty for Params {}

struct ParamSpec {
    id: Identifier,
    // TODO: implement type system
    param_type: String,
    doc: Doc,
    doc_ref: DocRef,
    enum_type: Enum,
}
