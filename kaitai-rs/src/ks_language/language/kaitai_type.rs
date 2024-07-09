// Enumeration representing all existing types
#[derive(Debug)]
pub enum PureType {
    UnsignedInteger(u8),
    SignedInteger(u8),
    BitSizedInteger(u8),
    FloatingPoint(u8),
    ByteArray,
    String,
    StringZ,
    Boolean,
    UserType(String), // Placeholder for user-defined type
    ArbitraryStruct,
    IOStream,
    AnyType,
}

// Type structure
#[derive(Debug)]
pub struct Type {
    // The pure type associated with the Type instance
    pub pure_type: PureType,
    // Indicates whether the type is an array
    pub is_array: bool,
}

/// Parses an unsigned integer of the given size and returns a Vec<u8> of the specified size
pub fn parse_unsigned_integer(data: &[u8], size: usize) -> Vec<u8> {
    let mut value = 0;
    for i in 0..size {
        value |= (data[i] as u64) << (i * 8);
    }
    let mut result = vec![0; size];
    result.copy_from_slice(&value.to_le_bytes()[..size]);
    result
}

/// Parses a null-terminated string (or with a custom terminator)
/// Returns the parsed string as a Vec<u8> up to and including the terminator if size is not specified,
/// or up to the specified size if provided, ignoring the terminator
pub fn parse_strz(data: &[u8], size: Option<usize>, terminator: u8) -> Vec<u8> {
    let end_pos = if let Some(specified_size) = size {
        // Use the specified size, ignoring the terminator
        std::cmp::min(specified_size, data.len())
    } else {
        // No size specified, find the terminator position
        if let Some(terminator_pos) = data.iter().position(|&x| x == terminator) {
            terminator_pos + 1 // Include the terminator
        } else {
            data.len() // Default to the entire length of the data
        }
    };

    // Return the string up to end_pos
    data[..end_pos].to_vec()
}
