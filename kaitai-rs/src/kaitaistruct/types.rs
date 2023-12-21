use crate::kaitaistruct::doc::Doc;
use crate::kaitaistruct::doc_ref::DocRef;
use crate::kaitaistruct::enums::Enums;
use crate::kaitaistruct::kaitai_property::KaitaiProperty;
use crate::kaitaistruct::meta::Meta;
use crate::kaitaistruct::params::Params;
use crate::kaitaistruct::seq::Seq;

/// Type struct
pub struct Types {
    types: Vec<TypeSpec>,
}

struct TypeSpec {
    meta: Meta,
    params: Params,
    seq: Seq,
    type_types: Types,
    type_enums: Enums,
    // TODO : Replace with instance struct
    instances: String,
    doc: Doc,
    doc_ref: DocRef,
}

impl KaitaiProperty for Types {}
