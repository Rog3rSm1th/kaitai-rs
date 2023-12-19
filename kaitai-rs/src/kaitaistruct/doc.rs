use crate::kaitaistruct::kaitai_property::KaitaiProperty;

/// Doc property struct
///
/// Contains:
/// 1. A description of the Kaitai struct
pub struct Doc {
    pub description: Option<String>,
}

impl KaitaiProperty for Doc {}
