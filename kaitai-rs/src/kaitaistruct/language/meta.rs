use crate::config::config::Config;
use crate::errors::KaitaiError;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;
use crate::utils::utils::validate_values;
use regex::Regex;

// Meta struct, representing metadata
pub struct Meta {
    // Identifier information
    identifier: Identifier,
    // Title of the Kaitai Struct
    title: String,
    // File extension information
    application: Application,
    // Cross-referencing details
    file_extension: FileExtension,
    // Cross-referencing details
    xref: XRef,
    // License type
    license: String,
    // Kaitai Struct version
    ks_version: f64,
    // Boolean flag indicating whether KS debug mode is enabled
    ks_debug: bool,
    // Boolean flag indicating whether KS opaque types are used
    ks_opaque_types: bool,
    // Import information
    imports: Imports,
    // Encoding used in the KS file
    encoding: String,
    // Endian used in the KS (le/be)
    endian: Endian,
}

impl KaitaiProperty for Meta {}

// Application struct to represent application information
struct Application {
    values: Vec<String>,
}

impl Application {
    fn new(values: Vec<String>) -> Self {
        Application { values }
    }

    // Getter method to retrieve application values
    fn get_values(&self) -> &Vec<String> {
        &self.values
    }
}

// Define the FileExtension struct to represent file extension information
struct FileExtension {
    values: Vec<String>,
}

impl FileExtension {
    fn new(values: Vec<String>) -> Self {
        FileExtension { values }
    }

    // Getter method to retrieve file extension values
    fn get_values(&self) -> &Vec<String> {
        &self.values
    }
}

// ForensicWiki struct to represent ForensicWiki information
struct ForensicWiki {
    value: Vec<String>,
}

impl ForensicWiki {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the media wiki page pattern
        validate_values(
            &values,
            Config::MEDIA_WIKI_PAGE_NAME_PATTERN,
            KaitaiError::BadWikiPageName,
        )?;

        Ok(ForensicWiki { value: values })
    }

    // Getter method to retrieve ForensicWiki values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// ISO struct to represent ISO information
struct ISO {
    value: Vec<String>,
}

impl ISO {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the ISO pattern
        validate_values(&values, Config::ISO_IDENTIFIER_PATTERN, KaitaiError::BadISO)?;

        Ok(ISO { value: values })
    }

    // Getter method to retrieve ISO values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// JustSolve struct to represent JustSolve information
struct JustSolve {
    value: Vec<String>,
}

impl JustSolve {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the media wiki page pattern
        validate_values(
            &values,
            Config::MEDIA_WIKI_PAGE_NAME_PATTERN,
            KaitaiError::BadJustSolve,
        )?;

        Ok(JustSolve { value: values })
    }

    // Getter method to retrieve JustSolve values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// LocIdentifier struct to represent LocIdentifier information
struct LocIdentifier {
    value: Vec<String>,
}

impl LocIdentifier {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the loc identifier pattern
        validate_values(
            &values,
            Config::LOC_IDENTIFIER_PATTERN,
            KaitaiError::BadLocIdentifier,
        )?;

        Ok(LocIdentifier { value: values })
    }

    // Getter method to retrieve LocIdentifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// MIMEType struct to represent MIMEType information
struct MIMEType {
    value: Vec<String>,
}

impl MIMEType {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the MIME type pattern
        validate_values(
            &values,
            Config::MIME_TYPE_PATTERN,
            KaitaiError::BadWikiPageName,
        )?;

        Ok(MIMEType { value: values })
    }

    // Getter method to retrieve MIMEType values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// PronomIdentifier struct to represent PronomIdentifier information
struct PronomIdentifier {
    value: Vec<String>,
}

impl PronomIdentifier {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the Pronom identifier pattern
        validate_values(
            &values,
            Config::PRONOM_IDENTIFIER_PATTERN,
            KaitaiError::BadPronomIdentifier,
        )?;

        Ok(PronomIdentifier { value: values })
    }

    // Getter method to retrieve PronomIdentifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// RFCIdentifier struct to represent RFCIdentifier information
struct RFCIdentifier {
    value: Vec<String>,
}

impl RFCIdentifier {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the RFC identifier pattern
        validate_values(
            &values,
            Config::RFC_IDENTIFIER_PATTERN,
            KaitaiError::BadRFCIdentifier,
        )?;

        Ok(RFCIdentifier { value: values })
    }

    // Getter method to retrieve RFCIdentifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// WikiDataIdentifier struct to represent WikiDataIdentifier information
struct WikiDataIdentifier {
    value: Vec<String>,
}

impl WikiDataIdentifier {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the WikiData identifier pattern
        validate_values(&values, Config::IMPORT_PATTERN, KaitaiError::BadImport)?;

        Ok(WikiDataIdentifier { value: values })
    }

    // Getter method to retrieve WikiDataIdentifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// XRef struct to represent cross-referencing information
struct XRef {
    forensic_wiki: ForensicWiki,
    iso: ISO,
    justsolve: JustSolve,
    loc: LocIdentifier,
    mime: MIMEType,
    pronom: PronomIdentifier,
    rfc: RFCIdentifier,
    wikidata: WikiDataIdentifier,
}

// Enum to represent the possible types of KsVersion
pub enum KsVersionValue {
    String(String),
    Number(u64),
}

// Imports struct to represent import information
struct Imports {
    value: Vec<String>,
}

impl Imports {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        validate_values(&values, Config::IMPORT_PATTERN, KaitaiError::BadImport)?;

        Ok(Imports { value: values })
    }

    // Getter method to retrieve Imports values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// Enum to represent the possible endian types
pub enum EndianEnum {
    Le,
    Be,
}

// Enum to represent various scalar types
pub enum AnyScalar {
    Str(String),
    Number(f64),
    Bool(bool),
    Integer(i64),
    Null,
}

// Define the Endian struct to represent endian information
pub struct Endian {
    switch_on: AnyScalar,
    endian: EndianEnum,
}
