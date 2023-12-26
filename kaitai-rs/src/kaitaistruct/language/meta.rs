use crate::config::config::Config;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::utils::utils::validate_values;
use std::io;

// Meta struct, representing metadata
#[derive(Debug)]
pub struct Meta {
    // Identifier information
    identifier: Option<Identifier>,
    // Title of the Kaitai Struct
    title: Option<String>,
    // File extension information
    application: Option<Application>,
    // Cross-referencing details
    file_extension: Option<FileExtension>,
    // Cross-referencing details
    xref: Option<XRef>,
    // License type
    license: Option<String>,
    // Kaitai Struct version
    ks_version: Option<f64>,
    // Boolean flag indicating whether KS debug mode is enabled
    ks_debug: Option<bool>,
    // Boolean flag indicating whether KS opaque types are used
    ks_opaque_types: Option<bool>,
    // Import information
    imports: Option<Imports>,
    // Encoding used in the KS file
    encoding: Option<String>,
    // Endian used in the KS (le/be)
    endian: Option<Endian>,
}

impl Meta {
    // Constructor for Meta struct with all fields set to None
    pub fn new() -> Self {
        Meta {
            identifier: None,
            title: None,
            application: None,
            file_extension: None,
            xref: None,
            license: None,
            ks_version: None,
            ks_debug: None,
            ks_opaque_types: None,
            imports: None,
            encoding: None,
            endian: None,
        }
    }
}

// Application struct to represent application information
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
struct ForensicWiki {
    value: Vec<String>,
}

impl ForensicWiki {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the media wiki page pattern
        validate_values(&values, Config::MEDIA_WIKI_PAGE_NAME_PATTERN);

        Ok(ForensicWiki { value: values })
    }

    // Getter method to retrieve ForensicWiki values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// ISO struct to represent ISO information
#[derive(Debug)]
struct ISO {
    value: Vec<String>,
}

impl ISO {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the ISO pattern
        validate_values(&values, Config::ISO_IDENTIFIER_PATTERN);

        Ok(ISO { value: values })
    }

    // Getter method to retrieve ISO values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// JustSolve struct to represent JustSolve information
#[derive(Debug)]
struct JustSolve {
    value: Vec<String>,
}

impl JustSolve {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the media wiki page pattern
        validate_values(&values, Config::MEDIA_WIKI_PAGE_NAME_PATTERN);

        Ok(JustSolve { value: values })
    }

    // Getter method to retrieve JustSolve values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// LocIdentifier struct to represent LocIdentifier information
#[derive(Debug)]
struct LocIdentifier {
    value: Vec<String>,
}

impl LocIdentifier {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the loc identifier pattern
        validate_values(&values, Config::LOC_IDENTIFIER_PATTERN);

        Ok(LocIdentifier { value: values })
    }

    // Getter method to retrieve LocIdentifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// MIMEType struct to represent MIMEType information
#[derive(Debug)]
struct MIMEType {
    value: Vec<String>,
}

impl MIMEType {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the MIME type pattern
        validate_values(&values, Config::MIME_TYPE_PATTERN);

        Ok(MIMEType { value: values })
    }

    // Getter method to retrieve MIMEType values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// PronomIdentifier struct to represent PronomIdentifier information
#[derive(Debug)]
struct PronomIdentifier {
    value: Vec<String>,
}

impl PronomIdentifier {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the Pronom identifier pattern
        validate_values(&values, Config::PRONOM_IDENTIFIER_PATTERN);

        Ok(PronomIdentifier { value: values })
    }

    // Getter method to retrieve PronomIdentifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// RFCIdentifier struct to represent RFCIdentifier information
#[derive(Debug)]
struct RFCIdentifier {
    value: Vec<String>,
}

impl RFCIdentifier {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the RFC identifier pattern
        validate_values(&values, Config::RFC_IDENTIFIER_PATTERN);

        Ok(RFCIdentifier { value: values })
    }

    // Getter method to retrieve RFCIdentifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// WikiDataIdentifier struct to represent WikiDataIdentifier information
#[derive(Debug)]
struct WikiDataIdentifier {
    value: Vec<String>,
}

impl WikiDataIdentifier {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the WikiData identifier pattern
        validate_values(&values, Config::IMPORT_PATTERN);

        Ok(WikiDataIdentifier { value: values })
    }

    // Getter method to retrieve WikiDataIdentifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// XRef struct to represent cross-referencing information
#[derive(Debug)]
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
#[derive(Debug)]
pub enum KsVersionValue {
    String(String),
    Number(u64),
}

// Imports struct to represent import information
#[derive(Debug)]
struct Imports {
    value: Vec<String>,
}

impl Imports {
    fn new(values: Vec<String>) -> Result<Self, io::Error> {
        validate_values(&values, Config::IMPORT_PATTERN);

        Ok(Imports { value: values })
    }

    // Getter method to retrieve Imports values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

// Enum to represent the possible endian types
#[derive(Debug)]
pub enum EndianEnum {
    Le,
    Be,
}

// Enum to represent various scalar types
#[derive(Debug)]
pub enum AnyScalar {
    Str(String),
    Number(f64),
    Bool(bool),
    Integer(i64),
    Null,
}

// Define the Endian struct to represent endian information
#[derive(Debug)]
pub struct Endian {
    switch_on: AnyScalar,
    endian: EndianEnum,
}
