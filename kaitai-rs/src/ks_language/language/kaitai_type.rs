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
