// Enumeration representing all existing types
#[derive(Debug)]
pub enum PureType {
    UnsignedInteger(u8),
    SignedInteger(u8),
    BitSizedInteger(u8),
    FloatingPoint(u8),
    ByteArray,
    String,
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

// Parses the data as an unsigned integer of the given size and returns a Vec<u8> of the specified size
pub fn parse_unsigned_integer(data: &[u8], size: usize) -> Vec<u8> {
    let mut value = 0;
    for i in 0..size {
        value |= (data[i] as u64) << (i * 8);
    }
    let mut result = vec![0; size];
    result.copy_from_slice(&value.to_le_bytes()[..size]);
    result
}
