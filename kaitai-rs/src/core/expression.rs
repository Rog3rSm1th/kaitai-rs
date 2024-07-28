use crate::core::ast::AST;
use pest::Parser;
use pest_derive::Parser;
use std::borrow::Borrow;

#[derive(Parser)]
#[grammar = "./core/expr.pest"]
struct ExprParser;

/// Converts the first 4 bytes of a Vec<u8> to an i32
fn vec_u8_to_i32(vec: &Vec<u8>) -> Result<i32, String> {
    if vec.len() < 4 {
        return Err(String::from("The vector must contain at least 4 elements"));
    }

    // Take the first 4 bytes and convert them to i32
    let bytes: [u8; 4] = [vec[0], vec[1], vec[2], vec[3]];
    let integer = i32::from_le_bytes(bytes);

    Ok(integer)
}

/// Parses an identifier and returns the corresponding i32 value from the AST
fn parse_identifier(ast: &AST, identifier: &str) -> Option<i32> {
    // Look up the node in the AST by identifier
    if let Some(node_ref) = ast.clone().get_node_by_id(identifier) {
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

/// Evaluates a kaitai language expression against an Abstract Syntax Tree (AST) of Vec<u8> nodes and returns an i32 result
/// TODO: handle more complex expressions, for now we only evaluate expressions composed solely of an integer or an identifier
pub fn evaluate(ast: &AST, expr: &str) -> i32 {
    // Try parsing an identifier
    if let Ok(pairs) = ExprParser::parse(Rule::identifier, expr) {
        for pair in pairs {
            let identifier = pair.as_str();
            if let Some(value) = parse_identifier(ast, identifier) {
                return value;
            }
        }
    }

    // Try parsing an integer
    if let Ok(pairs) = ExprParser::parse(Rule::integer, expr) {
        for pair in pairs {
            let integer_str = pair.as_str();
            if let Some(integer_value) = parse_integer(integer_str) {
                return integer_value;
            }
        }
    }

    // Default return value in case of errors or unsupported expressions
    0
}
