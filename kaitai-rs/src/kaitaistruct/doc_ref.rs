use crate::kaitaistruct::kaitai_property::KaitaiProperty;

/// DocRef property struct
///
/// Contains:
/// 1. URL as text,
/// 2. An arbitrary string
pub struct DocRef {
    pub url: Option<String>,
    pub arbitrary_string: Option<String>,
}

impl KaitaiProperty for DocRef {}
