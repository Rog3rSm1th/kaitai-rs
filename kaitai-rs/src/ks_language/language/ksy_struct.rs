use crate::ks_language::language::doc::Doc;
use crate::ks_language::language::doc_ref::DocRef;
use crate::ks_language::language::enums::Enums;
use crate::ks_language::language::instances::Instances;
use crate::ks_language::language::meta::Meta;
use crate::ks_language::language::params::Params;
use crate::ks_language::language::seq::Seq;
use crate::ks_language::language::types::Types;

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
