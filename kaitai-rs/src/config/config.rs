pub struct Config;

impl Config {
    // Regular expression pattern for identifiers
    pub const IDENTIFIER_PATTERN: &'static str = concat!(r"^[a-z][a-z0-9_]*$");

    // Regular expression pattern for MediaWiki page names
    pub const MEDIA_WIKI_PAGE_NAME_PATTERN: &'static str =
        concat!(r"^([a-zA-Z0-9$\\-._~+!*'(),@&;:\\/]|%[0-9a-fA-F]{2})+$");

    // Regular expression pattern for ISO identifiers
    pub const ISO_IDENTIFIER_PATTERN: &'static str = concat!(r"^[1-9]\d*(-[0-9]+)?:(19|20)\d{2}$");

    // Regular expression pattern for MIME types
    pub const MIME_TYPE_PATTERN: &'static str = concat!(
        r"^(application|audio|font|image|model|text|video)/",
        r"([a-zA-Z0-9]+[.\-_+])*[a-zA-Z0-9]+[.\-_+]?$"
    );

    // Regular expression pattern for LOC identifiers
    pub const LOC_IDENTIFIER_PATTERN: &'static str = concat!(r"^fdd\\d{6}$");

    // Regular expression pattern for PRONOM identifiers
    pub const PRONOM_IDENTIFIER_PATTERN: &'static str = concat!(r"^(x-)?fmt\\/\\d+$");

    // Regular expression pattern for RFC identifiers
    pub const RFC_IDENTIFIER_PATTERN: &'static str = concat!(r"^[1-9]\\d*$");

    // Regular expression pattern for WikiData identifiers
    pub const WIKI_DATA_IDENTIFIER_PATTERN: &'static str = concat!(r"^Q[1-9]\\d*$");

    // Regular expression pattern for imports
    pub const IMPORT_PATTERN: &'static str = concat!(r"^(.*/)?[a-z][a-z0-9_]*$");

    // Regular expression pattern for enum names
    pub const ENUM_NAME_PATTERN: &'static str = concat!(r"^[a-z][a-z0-9_]*$");

    // Regular expression pattern for type names
    pub const TYPE_NAME_PATTERN: &'static str =
        concat!(r"^([a-z][a-z0-9_]*::)*[a-z][a-z0-9_]*(\(.+\))?$");

    // Regular expression pattern for processes
    pub const PROCESS_PATTERN: &'static str = concat!(r"^zlib|(xor|rol|ror)\(.*\)$");

    // Regular expression pattern for DocRef
    pub const DOCREF_PATTERN: &'static str = concat!(
        r"^(?P<URL>https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*))",
        r"(( +)?(?P<arbitrary_string>.+))?"
    );
}
