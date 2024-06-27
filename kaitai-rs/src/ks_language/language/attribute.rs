use crate::ks_language::language::doc::Doc;
use crate::ks_language::language::doc_ref::DocRef;
use crate::ks_language::language::enums::Enum;
use crate::ks_language::language::kaitai_type::Type;

// Attribute struct definition
#[allow(dead_code)]
#[derive(Debug)]
pub struct Attribute {
    // Identifier for the attribute
    pub id: Option<String>,
    // Documentation for the attribute
    doc: Option<Doc>,
    // Reference to external documentation
    doc_ref: Option<DocRef>,
    // Contents of the attribute
    pub contents: Option<Vec<u8>>,
    // TODO: Implement type system
    pub seq_type: Option<Type>,
    // Repeat settings for the attribute
    repeat: Option<Repeat>,
    // Expression for the number of repetitions
    repeat_expr: Option<String>,
    // TODO: Implement the repeat_until parser & engine
    repeat_until: Option<String>,
    // Mark the attribute as optional
    optional_if: Option<String>,
    // Size of the attribute
    pub size: Option<String>,
    // Flag indicating whether size is until the end of the stream
    pub size_eos: bool,
    // Processing details for the attribute
    process: Option<Process>,
    // Enumeration associated with the attribute
    attribute_enum: Option<Enum>,
    // Encoding
    encoding: Option<String>,
    // Padding size to the right
    pad_right: Option<u8>,
    // string or byte array reading will stop when it encounters this byte
    terminator: Option<u8>,
    // Flag indicating whether to consume the attribute
    consume: bool,
    // Flag indicating whether to include the attribute in output
    include: bool,
    // Flag indicating whether an end-of-stream error should be raised
    eos_error: bool,
    // Position of the attribute in the stream
    pos: Option<String>,
    // Input/output association for the attribute
    io: Option<String>,
    // Value of the attribute
    value: Option<String>,
}

impl Attribute {
    /// Creates a new Attribute instance.
    pub fn new(
        id: Option<String>,
        doc: Option<Doc>,
        doc_ref: Option<DocRef>,
        contents: Option<Vec<u8>>,
        seq_type: Option<Type>,
        repeat: Option<Repeat>,
        repeat_expr: Option<String>,
        repeat_until: Option<String>,
        optional_if: Option<String>,
        size: Option<String>,
        size_eos: bool,
        process: Option<Process>,
        attribute_enum: Option<Enum>,
        encoding: Option<String>,
        pad_right: Option<u8>,
        terminator: Option<u8>,
        consume: bool,
        include: bool,
        eos_error: bool,
        pos: Option<String>,
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
            optional_if,
            size,
            size_eos,
            process,
            attribute_enum,
            encoding,
            pad_right,
            terminator,
            consume,
            include,
            eos_error,
            pos,
            io,
            value,
        }
    }

    /// Setters for the attribute fields
    pub fn set_contents(&mut self, contents: Vec<u8>) {
        self.contents = Some(contents);
    }

    pub fn set_seq_type(&mut self, seq_type: Type) {
        self.seq_type = Some(seq_type);
    }

    pub fn set_repeat(&mut self, repeat: Repeat) {
        self.repeat = Some(repeat);
    }

    pub fn set_repeat_expr(&mut self, repeat_expr: String) {
        self.repeat_expr = Some(repeat_expr);
    }

    pub fn set_repeat_until(&mut self, repeat_until: String) {
        self.repeat_until = Some(repeat_until);
    }

    pub fn set_optional_if(&mut self, optional_if: String) {
        self.optional_if = Some(optional_if);
    }

    pub fn set_size(&mut self, size: String) {
        self.size = Some(size);
    }

    pub fn set_size_eos(&mut self, size_eos: bool) {
        self.size_eos = size_eos;
    }

    pub fn set_process(&mut self, process: Process) {
        self.process = Some(process);
    }

    pub fn set_attribute_enum(&mut self, attribute_enum: Enum) {
        self.attribute_enum = Some(attribute_enum);
    }

    pub fn set_encoding(&mut self, encoding: String) {
        self.encoding = Some(encoding);
    }

    pub fn set_pad_right(&mut self, pad_right: u8) {
        self.pad_right = Some(pad_right);
    }

    pub fn set_terminator(&mut self, terminator: u8) {
        self.terminator = Some(terminator);
    }

    pub fn set_consume(&mut self, consume: bool) {
        self.consume = consume;
    }

    pub fn set_include(&mut self, include: bool) {
        self.include = include;
    }

    pub fn set_eos_error(&mut self, eos_error: bool) {
        self.eos_error = eos_error;
    }

    pub fn set_pos(&mut self, pos: String) {
        self.pos = Some(pos);
    }

    pub fn set_io(&mut self, io: String) {
        self.io = Some(io);
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
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
#[derive(Debug)]
pub struct Process {
    // Type of processing to be applied
    pub process_type: ProcessType,
    // TODO: Resolve the parameter
    pub parameter: String,
}
