use kaitai_rs::config::Config;
use kaitai_rs::utils::validate_values;

// This file contains unit tests for the generic validation function `validate_values`
// implemented in the `utils` module. The function is designed to validate a collection
// of strings against a specified regex pattern, returning an `io::Error` if any string
// fails to match the pattern.

// The tests cover various use cases by validating different types of identifiers and
// patterns defined in the `Config` module. Each test function focuses on a specific
// pattern and provides sets of valid and invalid strings for thorough validation.

#[test]
fn test_validate_identifiers() {
    // Define a set of valid identifiers
    let valid_identifiers = vec![
        "my_identifier".to_string(),
        "another_identifier123".to_string(),
        "ident_123".to_string(),
    ];

    // Define a set of invalid identifiers
    let invalid_identifiers = vec![
        "Invalid-Identifier".to_string(),
        "123_starts_with_number".to_string(),
        "_underscore_start".to_string(),
    ];

    // Test validation for valid identifiers
    assert!(validate_values(&valid_identifiers, Config::IDENTIFIER_PATTERN).is_ok());

    // Test validation for invalid identifiers
    assert!(validate_values(&invalid_identifiers, Config::IDENTIFIER_PATTERN).is_err());
}

#[test]
fn test_validate_media_wiki_page_names() {
    // Define a set of valid MediaWiki page names
    let valid_page_names = vec![
        "portable_network_graphics_(png)".to_string(),
        "exif".to_string(),
    ];

    // Define a set of invalid MediaWiki page names
    let invalid_page_names = vec!["^invalid".to_string()];

    // Test validation for valid MediaWiki page names
    assert!(validate_values(&valid_page_names, Config::MEDIA_WIKI_PAGE_NAME_PATTERN).is_ok());

    // Test validation for invalid MediaWiki page names
    assert!(validate_values(&invalid_page_names, Config::MEDIA_WIKI_PAGE_NAME_PATTERN).is_err());
}

#[test]
fn test_validate_iso() {
    // Define a set of valid ISOs
    let valid_iso = vec!["15948:2004".to_string(), "21320:2003".to_string()];

    // Define a set of invalid ISOs
    let invalid_iso = vec!["Invalid-ISO".to_string(), "123e".to_string()];

    // Test validation for valid ISOs
    assert!(validate_values(&valid_iso, Config::ISO_IDENTIFIER_PATTERN).is_ok());

    // Test validation for invalid ISOs
    assert!(validate_values(&invalid_iso, Config::ISO_IDENTIFIER_PATTERN).is_err());
}

#[test]
fn test_validate_mime_types() {
    // Define a set of valid MIME types
    let valid_mime_types = vec![
        "application/json".to_string(),
        "audio/mp3".to_string(),
        "image/png".to_string(),
    ];

    // Define a set of invalid MIME types
    let invalid_mime_types = vec![
        "invalid/mime-type".to_string(),
        "audio/mp3 invalid".to_string(),
        "image/png.".to_string(),
    ];

    // Test validation for valid MIME types
    assert!(validate_values(&valid_mime_types, Config::MIME_TYPE_PATTERN).is_ok());

    // Test validation for invalid MIME types
    assert!(validate_values(&invalid_mime_types, Config::MIME_TYPE_PATTERN).is_err());
}

#[test]
fn test_validate_loc() {
    // Define a set of valid locs
    let valid_locs = vec!["fdd123456".to_string(), "fdd789012".to_string()];

    // Define a set of invalid locs
    let invalid_locs = vec![
        "invalid/fdd123456".to_string(),
        "fdd123456 invalid".to_string(),
        "fdd12345".to_string(),
    ];

    // Test validation for valid locs
    assert!(validate_values(&valid_locs, Config::LOC_IDENTIFIER_PATTERN).is_ok());

    // Test validation for invalid locs
    assert!(validate_values(&invalid_locs, Config::LOC_IDENTIFIER_PATTERN).is_err());
}

#[test]
fn test_validate_pronom() {
    // Define a set of valid pronoms
    let valid_pronoms = vec!["fmt/801".to_string(), "x-fmt/456".to_string()];

    // Define a set of invalid pronoms
    let invalid_pronoms = vec![
        "invalid/fmt/123".to_string(),
        "fmt/123 invalid".to_string(),
        "x-fmt/invalid".to_string(),
    ];

    // Test validation for valid pronoms
    assert!(validate_values(&valid_pronoms, Config::PRONOM_IDENTIFIER_PATTERN).is_ok());

    // Test validation for invalid pronoms
    assert!(validate_values(&invalid_pronoms, Config::PRONOM_IDENTIFIER_PATTERN).is_err());
}

#[test]
fn test_validate_rfc() {
    // Define a set of valid RFCs
    let valid_rfcs = vec!["123456".to_string(), "987654".to_string()];

    // Define a set of invalid RFCs
    let invalid_rfcs = vec![
        "RFC123456".to_string(),
        "0".to_string(),
        "RFC-987654".to_string(),
    ];

    // Test validation for valid RFCs
    assert!(validate_values(&valid_rfcs, Config::RFC_IDENTIFIER_PATTERN).is_ok());

    // Test validation for invalid RFCs
    assert!(validate_values(&invalid_rfcs, Config::RFC_IDENTIFIER_PATTERN).is_err());
}

#[test]
fn test_validate_wiki_data_identifiers() {
    // Define a set of valid WikiData identifiers
    let valid_wiki_data_identifiers = vec!["Q123".to_string(), "Q98765".to_string()];

    // Define a set of invalid WikiData identifiers
    let invalid_wiki_data_identifiers = vec![
        "invalid/Q123".to_string(),
        "Q123 invalid".to_string(),
        "Q0".to_string(),
    ];

    // Test validation for valid WikiData identifiers
    assert!(validate_values(
        &valid_wiki_data_identifiers,
        Config::WIKI_DATA_IDENTIFIER_PATTERN
    )
    .is_ok());

    // Test validation for invalid WikiData identifiers
    assert!(validate_values(
        &invalid_wiki_data_identifiers,
        Config::WIKI_DATA_IDENTIFIER_PATTERN
    )
    .is_err());
}

#[test]
fn test_validate_imports() {
    // Define a set of valid imports
    let valid_imports = vec![
        "/network/rtp_packet".to_string(),
        "/network/ethernet_frame".to_string(),
    ];

    // Define a set of invalid imports
    let invalid_imports = vec![
        "/invalid_import".to_string(),
        "InvalidImport".to_string(),
        "invalid_import/".to_string(),
        "/".to_string(),
    ];

    // Test validation for valid imports
    assert!(validate_values(&valid_imports, Config::IMPORT_PATTERN).is_ok());

    // Test validation for invalid imports
    assert!(validate_values(&invalid_imports, Config::IMPORT_PATTERN).is_err());
}

#[test]
fn test_validate_enum_names() {
    // Define a set of valid enum names
    let valid_enum_names = vec![
        "my_enum".to_string(),
        "another_enum123".to_string(),
        "enum_123".to_string(),
    ];

    // Define a set of invalid enum names
    let invalid_enum_names = vec![
        "InvalidEnum".to_string(),
        "123_starts_with_number".to_string(),
        "_underscore_start".to_string(),
    ];

    // Test validation for valid enum names
    assert!(validate_values(&valid_enum_names, Config::ENUM_NAME_PATTERN).is_ok());

    // Test validation for invalid enum names
    assert!(validate_values(&invalid_enum_names, Config::ENUM_NAME_PATTERN).is_err());
}

#[test]
fn test_validate_type_names() {
    // Define a set of valid type names
    let valid_type_names = vec!["my_type".to_string(), "another_type123".to_string()];

    // Define a set of invalid type names
    let invalid_type_names = vec![
        "::InvalidType".to_string(),
        "InvalidType::".to_string(),
        "Invalid::Type::Name".to_string(),
        "nested::Invalid_Type".to_string(),
        "invalid type".to_string(),
    ];

    // Test validation for valid type names
    assert!(validate_values(&valid_type_names, Config::TYPE_NAME_PATTERN).is_ok());

    // Test validation for invalid type names
    assert!(validate_values(&invalid_type_names, Config::TYPE_NAME_PATTERN).is_err());
}

#[test]
fn test_validate_processes() {
    // Define a set of valid processes
    let valid_processes = vec![
        "zlib".to_string(),
        "xor(123)".to_string(),
        "rol(4)".to_string(),
        "ror(8)".to_string(),
    ];

    // Define a set of invalid processes
    let invalid_processes = vec![
        "invalid".to_string(),
        "xor(abc)".to_string(),
        "rol".to_string(),
        "ror()".to_string(),
    ];

    // Test validation for valid processes
    assert!(validate_values(&valid_processes, Config::PROCESS_PATTERN).is_ok());

    // Test validation for invalid processes
    assert!(validate_values(&invalid_processes, Config::PROCESS_PATTERN).is_err());
}

#[test]
fn test_validate_doc_refs() {
    // Define a set of valid DocRefs
    let valid_doc_refs = vec![
        "https://www.example.com/doc".to_string(),
        "http://example.org".to_string(),
        "https://kaitai.io/reference".to_string(),
        "https://www.example.com/doc arbitrary text".to_string(),
    ];

    // Define a set of invalid DocRefs
    let invalid_doc_refs = vec![
        "invalid-url".to_string(),
        "ftp://example.org".to_string(),
        "https://kaitai.io/reference invalid".to_string(),
    ];

    // Test validation for valid DocRefs
    assert!(validate_values(&valid_doc_refs, Config::DOCREF_PATTERN).is_ok());

    // Test validation for invalid DocRefs
    assert!(validate_values(&invalid_doc_refs, Config::DOCREF_PATTERN).is_err());
}
