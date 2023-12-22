// Enumeration representing all existing types
enum PureType {
    UnsignedInteger,
    SignedInteger,
    BitSizedInteger(u8),
    FloatingPoint,
    ByteArray,
    String,
    Boolean,
    UserType(String), // Placeholder for user-defined type
    ArbitraryStruct,
    IOStream,
    AnyType,
}

// Type structure
pub struct Type {
    // The pure type associated with the Type instance
    pure_type: PureType,
    // Indicates whether the type is an array
    is_array: bool,
}
