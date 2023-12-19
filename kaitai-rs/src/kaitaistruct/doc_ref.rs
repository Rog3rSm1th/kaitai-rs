use crate::kaitaistruct::kaitai_property::KaitaiProperty;

/// DocRef property struct
///
/// Contains:
/// 1. URL as text,
/// 2. arbitrary string
/// 3. or URL as text + space + arbitrary string.
pub struct DocRef {
    pub url: Option<String>,
    pub arbitrary_string: Option<String>,
}

impl KaitaiProperty for DocRef {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_docref_with_url() {
        let doc_ref_url = DocRef {
            url: Some("https://example.com/documentation".to_string()),
            arbitrary_string: None,
        };

        assert_eq!(
            doc_ref_url.url,
            Some("https://example.com/documentation".to_string())
        );
        assert_eq!(doc_ref_url.arbitrary_string, None);
    }

    #[test]
    fn create_docref_with_arbitrary_string() {
        let doc_ref_arbitrary_string = DocRef {
            url: None,
            arbitrary_string: Some("Some arbitrary string".to_string()),
        };

        assert_eq!(doc_ref_arbitrary_string.url, None);
        assert_eq!(
            doc_ref_arbitrary_string.arbitrary_string,
            Some("Some arbitrary string".to_string())
        );
    }

    #[test]
    fn create_docref_with_url_and_arbitrary_string() {
        let doc_ref_url_and_arbitrary_string = DocRef {
            url: Some("https://example.com/documentation".to_string()),
            arbitrary_string: Some("Some arbitrary string".to_string()),
        };

        assert_eq!(
            doc_ref_url_and_arbitrary_string.url,
            Some("https://example.com/documentation".to_string())
        );
        assert_eq!(
            doc_ref_url_and_arbitrary_string.arbitrary_string,
            Some("Some arbitrary string".to_string())
        );
    }
}
