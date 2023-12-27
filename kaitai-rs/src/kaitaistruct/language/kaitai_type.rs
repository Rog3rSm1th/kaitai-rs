// Enumeration representing all existing types
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct Type {
    // The pure type associated with the Type instance
    pure_type: PureType,
    // Indicates whether the type is an array
    is_array: bool,
}
