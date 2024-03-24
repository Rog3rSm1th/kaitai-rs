use crate::core::ast::AST;

/// Struct representing a Kaitai struct
#[allow(dead_code)]
#[derive(Debug)]
struct KaitaiStruct {
    data: Vec<u8>,
    ast: AST<Vec<u8>>,
}
