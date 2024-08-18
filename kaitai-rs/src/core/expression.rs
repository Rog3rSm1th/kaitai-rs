use crate::core::ast::AST;
use num_bigint::BigInt;
use num_traits::Num;
use pest::Parser;
use pest_derive::Parser;
use std::borrow::Borrow;

#[derive(Parser)]
#[grammar = "./core/expr.pest"]
struct ExprParser;

/// Parses an identifier and returns the corresponding Vec<u8> value from the AST
fn parse_identifier(ast: &AST, identifier: &str) -> Option<Vec<u8>> {
    // Look up the node in the AST by identifier
    if let Some(node_ref) = ast.get_node_by_id(identifier) {
        let node = node_ref.borrow();

        // Retrieve data from the node
        if let Some(data) = node.get_data() {
            return Some(data.clone());
        }
    }

    None
}

/// Parses an integer from a string and returns it as BigInt
fn parse_integer(integer_str: &str) -> Option<BigInt> {
    BigInt::from_str_radix(integer_str, 10).ok()
}

/// Parses a path_element constituted of only one element and returns the corresponding Vec<u8> value from the AST
fn parse_path_element(ast: &AST, path_element: &str) -> Option<Vec<u8>> {
    // Try parsing an identifier
    if let Ok(pairs) = ExprParser::parse(Rule::identifier, path_element) {
        for pair in pairs {
            let identifier = pair.as_str();
            if let Some(value) = parse_identifier(ast, identifier) {
                return Some(value);
            }
        }
    }

    // TODO: Handle method_call, user_defined_type and other path_element types
    todo!()
}

/// Parses a path constituted of only one element and returns the corresponding Vec<u8> value from the AST
fn parse_path(ast: &AST, path: &str) -> Option<Vec<u8>> {
    // Try parsing a path
    if let Ok(pairs) = ExprParser::parse(Rule::path, path) {
        for pair in pairs {
            let path_str = pair.as_str();
            if let Some(value) = parse_path_element(ast, path_str) {
                return Some(value);
            }
        }
    }

    // TODO: Handle more complex paths with indices or concatenation using a dot
    todo!()
}

/// Converts a Vec<u8> to a BigInt assuming big-endian encoding
/// TODO : Create a global endianness parameter
fn vec_u8_to_bigint(vec: &Vec<u8>) -> BigInt {
    let mut bigint = BigInt::from(0);
    for &byte in vec.iter().rev() {
        bigint = (bigint << 8) | BigInt::from(byte);
    }
    bigint
}

/// Evaluates a kaitai language expression against an Abstract Syntax Tree (AST) of Vec<u8> nodes and returns a BigInt result
/// Now handles expressions composed solely of a single node identifier or an integer
pub fn evaluate(ast: &AST, expr: &str) -> BigInt {
    // Try parsing an integer
    if let Some(integer_value) = parse_integer(expr) {
        return integer_value;
    }
    
    // Directly try to parse the expression as a single node identifier
    if let Some(raw_value) = parse_path(ast, expr) {
        // Convert the raw Vec<u8> to BigInt assuming little-endian encoding
        return vec_u8_to_bigint(&raw_value);
    }

    // Default return value in case of errors or unsupported expressions
    BigInt::from(0)
}
