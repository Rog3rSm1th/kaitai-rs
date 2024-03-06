use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::enums::Enum;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::language::kaitai_type::Type;

// Attribute struct definition
#[allow(dead_code)]
#[derive(Debug)]
pub struct Attribute {
    // Identifier for the attribute
    id: Identifier,
    // Documentation for the attribute
    doc: Option<Doc>,
    // Reference to external documentation
    doc_ref: Option<DocRef>,
    // Contents of the attribute
    contents: Option<Vec<String>>,
    // TODO: Implement type system
    seq_type: Option<Type>,
    // Repeat settings for the attribute
    repeat: Option<Repeat>,
    // Expression for the number of repetitions
    repeat_expr: Option<u32>,
    // TODO: Implement the repeat_until parser & engine
    repeat_until: Option<String>,
    // Size of the attribute
    size: Option<u32>,
    // Flag indicating whether size is until the end of the stream
    size_eos: Option<bool>,
    // Processing details for the attribute
    process: Option<Process>,
    // Enumeration associated with the attribute
    attribute_enum: Option<Enum>,
    // Padding size to the right
    pad_right: Option<u8>,
    // Flag indicating whether to consume the attribute
    consume: Option<bool>,
    // Flag indicating whether to include the attribute in output
    include: Option<bool>,
    // Flag indicating whether an end-of-stream error should be raised
    eos_error: Option<bool>,
    // Position of the attribute in the stream
    pos: Option<u32>,
    // Input/output association for the attribute
    io: Option<String>,
    // Value of the attribute
    value: Option<String>,
}

impl Attribute {
    /// Creates a new Attribute instance.
    pub fn new(
        id: Identifier,
        doc: Option<Doc>,
        doc_ref: Option<DocRef>,
        contents: Option<Vec<String>>,
        seq_type: Option<Type>,
        repeat: Option<Repeat>,
        repeat_expr: Option<u32>,
        repeat_until: Option<String>,
        size: Option<u32>,
        size_eos: Option<bool>,
        process: Option<Process>,
        attribute_enum: Option<Enum>,
        pad_right: Option<u8>,
        consume: Option<bool>,
        include: Option<bool>,
        eos_error: Option<bool>,
        pos: Option<u32>,
        io: Option<String>,
        value: Option<String>,
    ) -> Self {
        Attribute {
            id,
            doc,
            doc_ref,
            contents,
            seq_type,
            repeat,
            repeat_expr,
            repeat_until,
            size,
            size_eos,
            process,
            attribute_enum,
            pad_right,
            consume,
            include,
            eos_error,
            pos,
            io,
            value,
        }
    }
}

// Repeat enum for defining repetition behavior
#[derive(Debug)]
pub enum Repeat {
    // Repeat until the end of the current stream
    Eos,
    // Repeat as many times as specified in repeat-expr
    Expr,
    // Repeat until the expression in repeat-until becomes true
    Until,
}

// ProcessType enum for defining processing types
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Process {
    // Type of processing to be applied
    process_type: ProcessType,
    // TODO: Resolve the parameter
    parameter: String,
}
