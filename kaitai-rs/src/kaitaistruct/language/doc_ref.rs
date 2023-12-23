use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;

/// DocRef property struct
#[derive(Debug)]
pub struct DocRef {
    // A doc ref element contaning an url + arbitrary string
    content: Vec<DocRefElement>
}

#[derive(Debug)]
struct DocRefElement {
    // DocRef URL
    pub url: Option<String>,
    // Arbitrary string
    pub arbitrary_string: Option<String>,
}

impl KaitaiProperty for DocRef {}
