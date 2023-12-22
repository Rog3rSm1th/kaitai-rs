use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;

/// DocRef property struct
pub struct DocRef {
    // DocRef URL
    pub url: Option<String>,
    // Arbitrary string
    pub arbitrary_string: Option<String>,
}

impl KaitaiProperty for DocRef {}
