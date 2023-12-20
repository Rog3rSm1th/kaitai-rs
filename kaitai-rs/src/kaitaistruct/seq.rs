use crate::kaitaistruct::attributes::Identifier;
use crate::kaitaistruct::doc::Doc;
use crate::kaitaistruct::doc_ref::DocRef;
use crate::kaitaistruct::kaitai_property::KaitaiProperty;

/// Seq property struct
pub struct Seq {
    id: Identifier,
    doc: Doc,
    doc_ref: DocRef,
    contents: Vec<String>,
    // TODO: implement type system
    seq_type: String,
    repeat: Repeat,
    repeat_expr: u32,
    // TODO: Implement the repeat_until parser & engine
    repeat_until: String,
    size: u32,
    size_eos: bool,
    process: Process,
}

impl KaitaiProperty for Seq {}

// Repeat struct
pub enum Repeat {
    // Repeat until the end of the current stream
    Eos,
    // Repeat as many times as specified in repeat-expr
    Expr,
    // Repeat until the expression in repeat-until becomes true
    Until,
}

// Process Types
pub enum ProcessType {
    // Zlib decompression
    Zlib,
    // Bitwise XOR
    Xor,
    // Left circular shift
    Rol,
    // Right circular shift
    Ror,
}

// Process struct
pub struct Process {
    process_type: ProcessType,
    // TODO :Resolve the parameter
    parameter: String,
}
