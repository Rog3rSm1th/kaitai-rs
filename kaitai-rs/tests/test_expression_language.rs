#[macro_use]
extern crate pest_derive;

#[cfg(test)]
mod tests {
    use pest::Parser;

    #[derive(Parser)]
    #[grammar = "./core/expr.pest"]
    struct KaitaiExpressionParser;

    // This file contains unit tests for the kaitai expression language pest grammar.
    // https://doc.kaitai.io/user_guide.html#_expression_language
    //
    // TODO : Add tests for invalid literals & expression

    #[test]
    // Test parsing valid identifiers
    fn test_identifier() {
        let valid_identifiers = vec![
            "my_identifier123",
            "abc",
            "abcdefghijklmnopqrstuvwxyz0123456789",
        ];

        for identifier in valid_identifiers {
            let result = KaitaiExpressionParser::parse(Rule::identifier, identifier);
            assert!(
                result.is_ok(),
                "Expected identifier '{}' to be valid",
                identifier
            );
        }
    }

    #[test]
    // Test parsing valid arithmetic operators
    fn test_arithmetic_operator() {
        let valid_operators = vec!["+", "-", "/", "*", "%"];

        for operator in valid_operators {
            let result = KaitaiExpressionParser::parse(Rule::arithmetic_operator, operator);
            assert!(
                result.is_ok(),
                "Expected operator '{}' to be valid",
                operator
            );
        }
    }

    #[test]
    // Test parsing valid hexadecimal integers
    fn test_hex_integer() {
        let valid_hex_integers = vec!["0xABC123", "0x0", "0xFFFFFFFF", "0xABC_DEF"];

        for hex_integer in valid_hex_integers {
            let result = KaitaiExpressionParser::parse(Rule::hex_integer, hex_integer);
            assert!(
                result.is_ok(),
                "Expected hexadecimal integer '{}' to be valid",
                hex_integer
            );
        }
    }

    #[test]
    // Test parsing valid binary integers
    fn test_bin_integer() {
        let valid_bin_integers = vec!["0b10101010", "0b0", "0b11111111", "0b1010_1010"];

        for bin_integer in valid_bin_integers {
            let result = KaitaiExpressionParser::parse(Rule::bin_integer, bin_integer);
            assert!(
                result.is_ok(),
                "Expected binary integer '{}' to be valid",
                bin_integer
            );
        }
    }

    #[test]
    // Test parsing valid octal integers
    fn test_octal_integer() {
        let valid_octal_integers = vec!["0o755", "0o0", "0o7777", "0o755_555"];

        for octal_integer in valid_octal_integers {
            let result = KaitaiExpressionParser::parse(Rule::octal_integer, octal_integer);
            assert!(
                result.is_ok(),
                "Expected octal integer '{}' to be valid",
                octal_integer
            );
        }
    }

    #[test]
    // Test parsing valid floating-point numbers
    fn test_float() {
        let valid_floats = vec![
            "1.23",
            "0.0",
            "123.456",
            "1.23_456",
            "1.234e56",
            "1.234e+56",
            "1.234e-56",
        ];

        for float in valid_floats {
            let result = KaitaiExpressionParser::parse(Rule::floating_point_number, float);
            assert!(
                result.is_ok(),
                "Expected floating-point number '{}' to be valid",
                float
            );
        }
    }

    #[test]
    // Test parsing valid boolean values
    fn test_boolean() {
        let valid_booleans = vec!["true", "false"];

        for boolean in valid_booleans {
            let result = KaitaiExpressionParser::parse(Rule::boolean, boolean);
            assert!(
                result.is_ok(),
                "Expected boolean value '{}' to be valid",
                boolean
            );
        }
    }

    #[test]
    // Test parsing valid arrays of integers
    fn test_integer_array() {
        let valid_integer_arrays = vec![
            "[1, 2, 3]",
            "[0]",
            "[1, 2, 3, 4, 5]",
            "[1, 2, 3, 4, 5, 6, 7, 8, 9]",
        ];

        for integer_array in valid_integer_arrays {
            let result = KaitaiExpressionParser::parse(Rule::integer_array, integer_array);
            assert!(
                result.is_ok(),
                "Expected integer array '{}' to be valid",
                integer_array
            );
        }
    }

    #[test]
    // Test parsing valid user-defined types
    fn test_user_defined_type() {
        let valid_user_defined_types = vec!["_root", "_parent", "_io"];

        for user_defined_type in valid_user_defined_types {
            let result = KaitaiExpressionParser::parse(Rule::user_defined_type, user_defined_type);
            assert!(
                result.is_ok(),
                "Expected user-defined type '{}' to be valid",
                user_defined_type
            );
        }
    }

    #[test]
    // Test parsing a basic arithmetic operation
    fn test_parse_arithmetic_operation() {
        let input = "1 + 1";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }

    #[test]
    // Test parsing a user_defined type
    fn test_parse_user_defined_type() {
        let input = "_root.mo";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }

    #[test]
    // Test parsing a ternary operator
    fn test_parse_ternary_operator() {
        let input = "disk_type.to_i & 0x01 != 0 ? 40 : 80";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }

    #[test]
    // Test parsing nested ternary operators
    fn test_parse_nested_ternary_operators() {
        let input = "(t.to_i == 0) ? s2 : (s1.to_i == 0 ? 1 : 0)";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }

    #[test]
    // Test parsing an array at a given index
    fn test_parse_array_index() {
        let input = "block_groups[0]";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }

    #[test]
    // Test parsing a complex arithmetic expression
    fn test_parse_arithmetic_expression() {
        let input = "b1 | (b2 << 8) | (b3 << 16)";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }

    #[test]
    // Test parsing a complex arithmetic expression
    fn test_parse_arithmetic_expression_2() {
        let input = "(bpb.max_root_dir_rec * 32 + bpb.bytes_per_ls - 1) / bpb.bytes_per_ls";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }

    #[test]
    // Test parsing a comparison operation
    fn test_parse_comparison() {
        let input = "len_body != 0";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }

    #[test]
    // Test parsing an expression containing an expression operator
    fn test_parse_expression_operator() {
        let input = "_root.constant_pool[name_index - 1].cp_info.as<utf8_cp_info>.value";
        let result = KaitaiExpressionParser::parse(Rule::kaitai_expression, input);
        assert!(result.is_ok());
    }
}
