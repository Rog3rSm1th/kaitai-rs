use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::enums::Enum;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::language::kaitai_type::Type;

// Attribute struct definition
#[allow(dead_code)]
pub struct Attribute {
    // Identifier for the attribute
    id: Identifier,
    // Documentation for the attribute
    doc: Doc,
    // Reference to external documentation
    doc_ref: DocRef,
    // Contents of the attribute
    contents: Vec<String>,
    // TODO: Implement type system
    seq_type: Type,
    // Repeat settings for the attribute
    repeat: Repeat,
    // Expression for the number of repetitions
    repeat_expr: u32,
    // TODO: Implement the repeat_until parser & engine
    repeat_until: String,
    // Size of the attribute
    size: u32,
    // Flag indicating whether size is until the end of the stream
    size_eos: bool,
    // Processing details for the attribute
    process: Process,
    // Enumeration associated with the attribute
    attribute_enum: Enum,
    // Padding size to the right
    pad_right: u8,
    // Flag indicating whether to consume the attribute
    consume: bool,
    // Flag indicating whether to include the attribute in output
    include: bool,
    // Flag indicating whether an end-of-stream error should be raised
    eos_error: bool,
    // Position of the attribute in the stream
    pos: u32,
    // Input/output association for the attribute
    io: String,
    // Value of the attribute
    value: String,
}

// Repeat enum for defining repetition behavior
pub enum Repeat {
    // Repeat until the end of the current stream
    Eos,
    // Repeat as many times as specified in repeat-expr
    Expr,
    // Repeat until the expression in repeat-until becomes true
    Until,
}

// ProcessType enum for defining processing types
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

// Process struct definition
#[allow(dead_code)]
pub struct Process {
    // Type of processing to be applied
    process_type: ProcessType,
    // TODO: Resolve the parameter
    parameter: String,
}
