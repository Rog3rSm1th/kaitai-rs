use crate::core::ast::AST;

/// Struct representing a Kaitai struct
struct KaitaiStruct {
    data: Vec<u8>,
    ast: AST<Vec<u8>>,
}