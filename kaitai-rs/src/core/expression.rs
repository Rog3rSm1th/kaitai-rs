use crate::core::ast::AST;
use pest::Parser;
use pest_derive::Parser;
use std::borrow::Borrow;

#[derive(Parser)]
#[grammar = "./core/expr.pest"]
struct ExprParser;

/// Converts a Vec<u8> to an i32 assuming little-endian encoding
fn vec_u8_to_i32(vec: &Vec<u8>) -> Result<i32, String> {
    if vec.len() != 4 {
        return Err(String::from("The vector must contain exactly 4 elements"));
    }

    // Convert the entire Vec<u8> to i32 assuming little-endian encoding
    let integer = i32::from_le_bytes([vec[0], vec[1], vec[2], vec[3]]);

    Ok(integer)
}

/// Parses an identifier and returns the corresponding i32 value from the AST
fn parse_identifier(ast: &AST, identifier: &str) -> Option<i32> {
    // Look up the node in the AST by identifier
    if let Some(node_ref) = ast.get_node_by_id(identifier) {
        let node = node_ref.borrow();

        // Retrieve data from the node
        if let Some(data) = node.get_data() {
            // Convert data to i32 directly
            if let Ok(value) = vec_u8_to_i32(data) {
                return Some(value);
            }
        }
    }

    None
}

/// Parses an integer from a string and returns it as i32
fn parse_integer(integer_str: &str) -> Option<i32> {
    integer_str.parse::<i32>().ok()
}

/// Parses a path_element and returns the corresponding i32 value from the AST
fn parse_path_element(ast: &AST, path_element: &str) -> Option<i32> {
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

/// Evaluates a kaitai language expression against an Abstract Syntax Tree (AST) of Vec<u8> nodes and returns an i32 result
/// Now handles expressions composed solely of a single node identifier or an integer
pub fn evaluate(ast: &AST, expr: &str) -> i32 {
    // Try parsing an integer
    if let Some(integer_value) = parse_integer(expr) {
        return integer_value;
    }

    // Directly try to parse the expression as a single node identifier
    if let Some(value) = parse_path_element(ast, expr) {
        return value;
    }

    // Default return value in case of errors or unsupported expressions
    0
}
