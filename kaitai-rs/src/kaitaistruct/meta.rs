// Import necessary modules and dependencies
use crate::config::config::Config;
use crate::errors::KaitaiError;
use regex::Regex;

// Meta struct, representing metadata
struct Meta {
    pub identifier: Identifier,
    pub title: String,
    pub application: Application,
    pub file_extension: FileExtension,
    pub xref: XRef,
    pub license: String,
    pub ks_version: KsVersion,
    pub ks_debug: bool,
    pub ks_opaque_types: bool,
    pub imports: Imports,
    pub encoding: String,
    pub endian: Endian,
}

// Identifier struct to represent identifier
struct Identifier {
    value: Vec<String>,
}

impl Identifier {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the identifier pattern
        let regex =
            Regex::new(Config::IDENTIFIER_PATTERN).map_err(|_| KaitaiError::BadMetaIdentifier)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadMetaIdentifier);
            }
        }

        Ok(Identifier { value: values })
    }

    // Getter method to retrieve identifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}

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
        let regex = Regex::new(Config::MEDIA_WIKI_PAGE_NAME_PATTERN)
            .map_err(|_| KaitaiError::BadWikiPageName)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadWikiPageName);
            }
        }

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
        let regex = Regex::new(Config::ISO_IDENTIFIER_PATTERN).map_err(|_| KaitaiError::BadISO)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadISO);
            }
        }

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
        let regex = Regex::new(Config::MEDIA_WIKI_PAGE_NAME_PATTERN)
            .map_err(|_| KaitaiError::BadJustSolve)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadJustSolve);
            }
        }

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
        let regex = Regex::new(Config::LOC_IDENTIFIER_PATTERN)
            .map_err(|_| KaitaiError::BadLocIdentifier)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadLocIdentifier);
            }
        }

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
        let regex =
            Regex::new(Config::MIME_TYPE_PATTERN).map_err(|_| KaitaiError::BadWikiPageName)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadWikiPageName);
            }
        }

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
        let regex = Regex::new(Config::PRONOM_IDENTIFIER_PATTERN)
            .map_err(|_| KaitaiError::BadPronomIdentifier)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadPronomIdentifier);
            }
        }

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
        let regex = Regex::new(Config::RFC_IDENTIFIER_PATTERN)
            .map_err(|_| KaitaiError::BadRFCIdentifier)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadRFCIdentifier);
            }
        }

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
        let regex = Regex::new(Config::WIKI_DATA_IDENTIFIER_PATTERN)
            .map_err(|_| KaitaiError::BadWikiDataIdentifier)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadWikiDataIdentifier);
            }
        }

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

// KsVersion struct to represent Kaitai Struct version information
pub struct KsVersion {
    value: KsVersionValue,
}

impl KsVersion {
    pub fn new(value: KsVersionValue) -> KsVersion {
        KsVersion { value }
    }

    // Method to convert KsVersion to a string
    pub fn as_string(&self) -> String {
        match &self.value {
            KsVersionValue::String(version_str) => version_str.clone(),
            KsVersionValue::Number(version_num) => version_num.to_string(),
        }
    }
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
        let regex = Regex::new(Config::IMPORT_PATTERN).map_err(|_| KaitaiError::BadImport)?;

        for value in &values {
            if !regex.is_match(value) {
                return Err(KaitaiError::BadImport);
            }
        }

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
