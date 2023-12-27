use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::enums::Enums;
use crate::kaitaistruct::language::instances::Instances;
use crate::kaitaistruct::language::meta::Meta;
use crate::kaitaistruct::language::params::Params;
use crate::kaitaistruct::language::seq::Seq;
use crate::kaitaistruct::language::types::Types;

// KsyStruct struct representing the overall structure of a KSY file
#[allow(dead_code)]
pub struct KsyStruct {
    // Metadata
    meta: Meta,
    // Documentation
    doc: Doc,
    // Reference to external documentation
    doc_ref: DocRef,
    // Parameters specification
    params: Params,
    // Sequence specification
    seq: Seq,
    // Types specification
    types: Types,
    // Instances specification
    instances: Instances,
    // Enums specification
    enums: Enums,
}
