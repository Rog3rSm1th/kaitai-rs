use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum KaitaiError {
    // Invalid meta section identifier
    #[error("Meta identifier is not valid")]
    BadMetaIdentifier,

    // Invalid wiki page name
    #[error("Wiki page name is not valid")]
    BadWikiPageName,

    // Invalid ISO
    #[error("ISO is not valid")]
    BadISO,

    // Invalid Loc identifier
    #[error("Loc identifier is not valid")]
    BadLocIdentifier,

    // Invalid Loc identifier
    #[error("justsolve is not valid")]
    BadJustSolve,

    // Invalid pronom identifier
    #[error("Pronom identifier is not valid")]
    BadPronomIdentifier,

    // Invalid RFC identifier
    #[error("RFC identifier is not valid")]
    BadRFCIdentifier,

    // Invalid wiki data identifier
    #[error("RFC identifier is not valid")]
    BadWikiDataIdentifier,

    // Invalid import
    #[error("Import is not valid")]
    BadImport,

    // Invalid enum name
    #[error("Enum name is not valid")]
    BadEnumName,

    // Invalid DocRef
    #[error("doc-ref section is not valid")]
    BadDocRef,
}
