use crate::ks_language::language::doc::Doc;
use crate::ks_language::language::doc_ref::DocRef;
use crate::ks_language::language::enums::Enum;
use crate::ks_language::language::identifier::Identifier;
use crate::ks_language::language::kaitai_type::Type;

/// Params struct representing a list of params specifications
#[derive(Debug)]
pub struct Params {
    // List of params spec
    pub params_spec: Vec<ParamSpec>,
}

impl Params {
    /// Constructs a new Params instance with an empty list of ParamSpecs
    pub fn new() -> Self {
        Params {
            params_spec: Vec::new(),
        }
    }
}

impl Params {}

/// ParamSpec struct representing a parameter specification
#[derive(Debug)]
pub struct ParamSpec {
    // Identifier for the parameter
    pub id: Identifier,
    // TODO: implement type system
    pub param_type: Option<Type>,
    // Documentation for the parameter
    pub doc: Doc,
    // Reference to external documentation
    pub doc_ref: DocRef,
    // Enumeration associated with the parameter
    pub enum_type: Option<Enum>,
}

impl ParamSpec {
    /// Constructs a new ParamSpec instance with default values
    pub fn new() -> Self {
        ParamSpec {
            id: Identifier::new(),
            param_type: None,
            doc: Doc::new(),
            doc_ref: DocRef::new(),
            enum_type: None,
        }
    }

    /// Sets the type for the parameter
    pub fn set_param_type(&mut self, param_type: Type) {
        self.param_type = Some(param_type);
    }

    /// Sets the enumeration associated with the parameter
    pub fn set_enum_type(&mut self, enum_type: Enum) {
        self.enum_type = Some(enum_type);
    }
}
