use std::borrow::Borrow;

use num_bigint::BigInt;
use num_traits::Num;
use num_traits::ToPrimitive;
use pest::Parser;
use pest_derive::Parser;

use crate::core::ast::AST;

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

/// Parses a literal
fn parse_literal(ast: &AST, literal_str: &str) -> Option<BigInt> {
    // Try parsing a literal using the literal rule
    if let Ok(pairs) = ExprParser::parse(Rule::literal, literal_str) {
        for pair in pairs {
            // Iterate through the inner pairs if the current pair has any
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::integer => {
                        if let Some(integer_value) = parse_integer(inner_pair.as_str()) {
                            return Some(integer_value);
                        }
                    }
                    Rule::path => {
                        if let Some(raw_value) = parse_path(ast, inner_pair.as_str()) {
                            // Convert the raw Vec<u8> to BigInt assuming little-endian encoding
                            return Some(vec_u8_to_bigint(&raw_value));
                        }
                    }
                    // Handle floating point number parsing
                    Rule::floating_point_number => {
                        todo!();
                    }
                    // Handle boolean parsing
                    Rule::boolean => {
                        todo!();
                    }
                    // Handle string parsing
                    Rule::string => {
                        todo!();
                    }
                    // Handle array parsing
                    Rule::array => {
                        todo!();
                    }
                    _ => {}
                }
            }
        }
    }
    None
}

// Parses a binary expression
// TODO : Handle parentheses & order of operations
fn parse_binary_expression(ast: &AST, expr: &str) -> Option<BigInt> {
    if let Ok(pairs) = ExprParser::parse(Rule::binary_expression, expr) {
        let mut terms = Vec::new();
        let mut operators = Vec::new();

        for pair in pairs {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::primary_expression => {
                        let deepest = get_deepest_inner_pair(inner_pair);
                        if let Some(value) = parse_primary_expression(ast, deepest.as_str()) {
                            terms.push(value);
                        }
                    }
                    Rule::operator => {
                        let deepest = get_deepest_inner_pair(inner_pair);
                        operators.push(deepest.as_str());
                    }
                    _ => {}
                }
            }
        }

        // If there's only one term, return it directly
        if terms.len() == 1 {
            return Some(terms[0].clone());
        }

        // Evaluate the expression left-to-right using the operators
        let mut result = terms[0].clone();
        for (i, op) in operators.iter().enumerate() {
            let rhs = &terms[i + 1];
            result = match *op {
                "+" => result + rhs,
                "-" => result - rhs,
                "*" => result * rhs,
                "/" => result / rhs,
                "%" => result % rhs,
                "<<" => result << rhs.to_usize().unwrap(),
                ">>" => result >> rhs.to_usize().unwrap(),
                "&" => result & rhs,
                "|" => result | rhs,
                "^" => result ^ rhs,
                "==" => return Some((result == *rhs).into()),
                "!=" => return Some((result != *rhs).into()),
                "<" => return Some((result < *rhs).into()),
                "<=" => return Some((result <= *rhs).into()),
                ">" => return Some((result > *rhs).into()),
                ">=" => return Some((result >= *rhs).into()),
                "and" => {
                    return Some((result != BigInt::from(0) && rhs != &BigInt::from(0)).into())
                }
                "or" => return Some((result != BigInt::from(0) || rhs != &BigInt::from(0)).into()),
                "not" => return Some((result == BigInt::from(0)).into()),
                _ => return None,
            };
        }
        return Some(result);
    }
    None
}

/// Recursive function to get the deepest inner pair
fn get_deepest_inner_pair(pair: pest::iterators::Pair<Rule>) -> pest::iterators::Pair<Rule> {
    let mut current_pair = pair;
    while let Some(inner) = current_pair.clone().into_inner().next() {
        current_pair = inner;
    }
    current_pair
}

/// Parses a primary expression and returns its BigInt representation
fn parse_primary_expression(ast: &AST, expr: &str) -> Option<BigInt> {
    if let Ok(pairs) = ExprParser::parse(Rule::primary_expression, expr) {
        for pair in pairs {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::literal => {
                        if let Some(literal_value) = parse_literal(ast, inner_pair.as_str()) {
                            return Some(literal_value);
                        }
                    }
                    Rule::identifier => {
                        if let Some(raw_value) = parse_identifier(ast, inner_pair.as_str()) {
                            return Some(vec_u8_to_bigint(&raw_value));
                        }
                    }
                    Rule::expression => {
                        let expr_value = evaluate(ast, inner_pair.as_str());
                        return Some(expr_value);
                    }
                    _ => {}
                }
            }
        }
    }
    None
}

/// Evaluates a kaitai language expression against an AST of Vec<u8> nodes and returns a BigInt result
pub fn evaluate(ast: &AST, expr: &str) -> BigInt {
    if let Some(binary_value) = parse_binary_expression(ast, expr) {
        return binary_value;
    }

    BigInt::from(0)
}
