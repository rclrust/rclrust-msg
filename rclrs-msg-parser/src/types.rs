use std::fmt;

macro_rules! define_enum_from {
    ($into_t:ty, $from_t:ty, $path:path) => {
        impl From<$from_t> for $into_t {
            fn from(t: $from_t) -> Self {
                $path(t)
            }
        }
    };
}

/// A basic type according to the IDL specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicType {
    // signed integer type
    /// Rust: [i8], C++: `int8_t`
    I8,
    /// Rust: [i16], C++: `int16_t`
    I16,
    /// Rust: [i32], C++: `int32_t`
    I32,
    /// Rust: [i64], C++: `int64_t`
    I64,

    // unsigned integer type
    /// Rust: [u8], C++: `uint8_t`
    U8,
    /// Rust: [u16], C++: `uint16_t`
    U16,
    /// Rust: [u32], C++: `uint32_t`
    U32,
    /// Rust: [u64], C++: `uint64_t`
    U64,

    // floating point type
    /// Rust: [f32], C++: `float`
    F32,
    /// Rust: [f64], C++: `double`
    F64,
    // long double is not supported

    // boolean type
    /// Rust: [bool], C++: `bool`
    Bool,

    // duplicated type
    /// Rust: [u8], C++: `unsigned char`
    Char,
    /// Rust: [u8], C++: `unsigned char`
    Byte,
}

impl BasicType {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "uint8" => Self::U8,
            "uint16" => Self::U16,
            "uint32" => Self::U32,
            "uint64" => Self::U64,
            "int8" => Self::I8,
            "int16" => Self::I16,
            "int32" => Self::I32,
            "int64" => Self::I64,
            "float32" => Self::F32,
            "float64" => Self::F64,
            "bool" => Self::Bool,
            "char" => Self::Char,
            "byte" => Self::Byte,
            _ => {
                return None;
            }
        })
    }
}

/// A type identified by the name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedType(pub String);

impl fmt::Display for NamedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A type identified by a name in a namespaced scope
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespacedType {
    /// A package name which this type belongs to
    /// e.g. `std_msgs`
    pub package_name: String,
    /// A name of message
    /// e.g. `Bool`
    pub name: String,
}

impl fmt::Display for NamespacedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/msg/{}", self.package_name, self.name)
    }
}

/// A string type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericString {
    String,
    WString,
    BoundedString(usize),
    BoundedWString(usize),
}

impl From<GenericUnboundedString> for GenericString {
    fn from(t: GenericUnboundedString) -> Self {
        match t {
            GenericUnboundedString::String => Self::String,
            GenericUnboundedString::WString => Self::WString,
        }
    }
}

/// A generic unbounded string type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericUnboundedString {
    String,
    WString,
}

/// A type which can be used inside nested types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NestableType {
    BasicType(BasicType),
    NamedType(NamedType),
    NamespacedType(NamespacedType),
    GenericString(GenericString),
}

define_enum_from!(NestableType, BasicType, Self::BasicType);
define_enum_from!(NestableType, NamedType, Self::NamedType);
define_enum_from!(NestableType, NamespacedType, Self::NamespacedType);
define_enum_from!(NestableType, GenericString, Self::GenericString);

/// An array type with a static size
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
    /// The type of the elements
    pub value_type: NestableType,
    /// The number of elements in the array
    pub size: usize,
}

/// A sequence type with an unlimited number of elements
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    /// The type of the elements
    pub value_type: NestableType,
}

/// A sequence type with a maximum number of elements
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedSequence {
    /// The type of the elements
    pub value_type: NestableType,
    /// The maximum number of elements in the sequence
    pub max_size: usize,
}

/// A type which is available for member
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberType {
    BasicType(BasicType),
    NamedType(NamedType),
    NamespacedType(NamespacedType),
    GenericString(GenericString),
    Array(Array),
    Sequence(Sequence),
    BoundedSequence(BoundedSequence),
}

define_enum_from!(MemberType, BasicType, Self::BasicType);
define_enum_from!(MemberType, NamedType, Self::NamedType);
define_enum_from!(MemberType, NamespacedType, Self::NamespacedType);
define_enum_from!(MemberType, GenericString, Self::GenericString);
define_enum_from!(MemberType, Array, Self::Array);
define_enum_from!(MemberType, Sequence, Self::Sequence);
define_enum_from!(MemberType, BoundedSequence, Self::BoundedSequence);

impl From<NestableType> for MemberType {
    fn from(t: NestableType) -> Self {
        match t {
            NestableType::BasicType(t) => Self::BasicType(t),
            NestableType::NamedType(t) => Self::NamedType(t),
            NestableType::NamespacedType(t) => Self::NamespacedType(t),
            NestableType::GenericString(t) => Self::GenericString(t),
        }
    }
}

/// A primitive type which can be used for constant
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    BasicType(BasicType),
    GenericUnboundedString(GenericUnboundedString),
}

define_enum_from!(PrimitiveType, BasicType, Self::BasicType);
define_enum_from!(
    PrimitiveType,
    GenericUnboundedString,
    Self::GenericUnboundedString
);

/// An array type of a primitive type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveArray {
    /// The type of the elements
    pub value_type: PrimitiveType,
    /// The number of elements in the array
    pub size: usize,
}

/// A type which is available for constant
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstantType {
    BasicType(BasicType),
    GenericUnboundedString(GenericUnboundedString),
    PrimitiveArray(PrimitiveArray),
}

define_enum_from!(ConstantType, BasicType, Self::BasicType);
define_enum_from!(
    ConstantType,
    GenericUnboundedString,
    Self::GenericUnboundedString
);
define_enum_from!(ConstantType, PrimitiveArray, Self::PrimitiveArray);

impl From<PrimitiveType> for ConstantType {
    fn from(t: PrimitiveType) -> Self {
        match t {
            PrimitiveType::BasicType(t) => Self::BasicType(t),
            PrimitiveType::GenericUnboundedString(t) => Self::GenericUnboundedString(t),
        }
    }
}
