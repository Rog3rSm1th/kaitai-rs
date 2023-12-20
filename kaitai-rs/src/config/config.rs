pub struct Config;

impl Config {
    /// Meta Identifiers

    // Regular expression patterns for identifiers
    pub const IDENTIFIER_PATTERN: &'static str = concat!(r"^[a-z][a-z0-9_]*$");

    // Regular expression patterns for MediaWiki page names
    pub const MEDIA_WIKI_PAGE_NAME_PATTERN: &'static str =
        concat!(r"^([a-zA-Z0-9$\\-._~+!*'(),@&;:\\/]|%[0-9a-fA-F]{2})+$");

    // Regular expression patterns for ISO identifiers
    pub const ISO_IDENTIFIER_PATTERN: &'static str = concat!(r"^[1-9]\d*(-[0-9]+)?:(19|20)\d{2}$");

    // Regular expression patterns for MIME types
    pub const MIME_TYPE_PATTERN: &'static str = concat!(
        r"^(application|audio|font|image|model|text|video)/",
        r"([a-zA-Z0-9]+[.\-_+])*[a-zA-Z0-9]+[.\-_+]?$"
    );

    // Regular expression patterns for loc identifiers
    pub const LOC_IDENTIFIER_PATTERN: &'static str = concat!(r"^fdd\\d{6}$");

    // Regular expression patterns for pronom identifiers
    pub const PRONOM_IDENTIFIER_PATTERN: &'static str = concat!(r"^(x-)?fmt\\/\\d+$");

    // Regular expression patterns for RFC identifiers
    pub const RFC_IDENTIFIER_PATTERN: &'static str = concat!(r"^[1-9]\\d*$");

    // Regular expression patterns for wiki data identifiers
    pub const WIKI_DATA_IDENTIFIER_PATTERN: &'static str = concat!(r"^Q[1-9]\\d*$");

    // Regular expression patterns for imports
    pub const IMPORT_PATTERN: &'static str = concat!(r"^(.*/)?[a-z][a-z0-9_]*$");

    // Enum name pattern
    pub const ENUM_NAME_PATTERN: &'static str = concat!(r"^[a-z][a-z0-9_]*$");

    // Type name pattern
    pub const TYPE_NAME_PATTERN: &'static str =
        concat!(r"^([a-z][a-z0-9_]*::)*[a-z][a-z0-9_]*(\(.+\))?$");

    // Process pattern
    pub const PROCESS_PATTERN: &'static str = concat!(r"^zlib|(xor|rol|ror)\(.*\)$");
}
