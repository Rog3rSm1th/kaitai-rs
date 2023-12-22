use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;

/// Doc property struct
pub struct Doc {
    // description of the Kaitai struct
    pub description: Option<String>,
}

impl KaitaiProperty for Doc {}
