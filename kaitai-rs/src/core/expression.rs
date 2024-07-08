use crate::core::ast::AST;
use pest::Parser;
use pest_derive::Parser;

use std::borrow::Borrow;

#[derive(Parser)]
#[grammar = "./core/expr.pest"]
struct ExprParser;

/// Converts the first 4 bytes of a Vec<u8> to an i32
fn vec_u8_to_i32(vec: Vec<u8>) -> Result<i32, String> {
    if vec.len() < 4 {
        return Err(String::from("The vector must contain at least 4 elements"));
    }

    // Take the first 4 bytes and convert them to i32
    let bytes: [u8; 4] = [vec[0], vec[1], vec[2], vec[3]];
    let integer = i32::from_le_bytes(bytes);

    Ok(integer)
}

/// Evaluates an kaitai language expression against an Abstract Syntax Tree (AST) of Vec<u8> nodes and returns an i32 result
/// TODO : handle more complex expressions, for now we only evaluate expressions composed solely of an integer or an identifier
pub fn evaluate(ast: &AST<Vec<u8>>, expr: &str) -> i32 {
    match ExprParser::parse(Rule::literal, expr) {
        Ok(pairs) => {
            for pair in pairs {
                // Iterate through the inner pairs of the literal
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::identifier => {
                            let identifier = inner_pair.as_str();

                            // Look up the node in the AST by identifier
                            if let Some(node_ref) = ast.clone().get_node_by_id(identifier) {
                                let node = node_ref.borrow();

                                // Retrieve data from the node
                                if let Some(data) = node.get_data() {
                                    // Convert data to i32 directly
                                    if let Ok(value) = vec_u8_to_i32(data.clone()) {
                                        return value;
                                    }
                                }
                            }
                        }
                        Rule::integer => {
                            // Parse integer directly
                            let integer_value: i32 = inner_pair.as_str().parse().unwrap();
                            return integer_value;
                        }
                        _ => {
                            println!("Unsupported rule: {:?}", inner_pair.as_rule());
                        }
                    }
                }
            }
        }
        Err(e) => println!("Failed to parse expression: {}", e),
    }

    // Default return value in case of errors or unsupported expressions
    0
}
