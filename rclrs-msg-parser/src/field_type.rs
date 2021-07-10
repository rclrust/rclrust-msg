#[derive(Debug, Clone)]
pub enum BasicType {
    // signed integer type
    I8,
    I16,
    I32,
    I64,

    // unsigned integer type
    U8,
    U16,
    U32,
    U64,

    // floating point
    F32,
    F64,
    // long double is not supported

    // character type
    Char,
    WChar,

    Bool,
    Byte,
}

#[derive(Debug, Clone)]
pub struct NamedType(String);

#[derive(Debug, Clone)]
pub struct String;

#[derive(Debug, Clone)]
pub struct WString;

#[derive(Debug, Clone)]
pub struct BoundedString(usize);

#[derive(Debug, Clone)]
pub struct BoundedWString(usize);

#[derive(Debug, Clone)]
pub enum NestableType {
    BasicType(BasicType),
    NamedType(NamedType),
    String(String),
    WString(WString),
    BoundedString(BoundedString),
    BoundedWString(BoundedWString),
}

#[derive(Debug, Clone)]
pub struct Array {
    pub value_type: NestableType,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub value_type: NestableType,
}

#[derive(Debug, Clone)]
pub struct BoundedSequence {
    pub value_type: NestableType,
    pub max_size: usize,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    BasicType(BasicType),
    NamedType(NamedType),
    String(String),
    WString(WString),
    BoundedString(BoundedString),
    BoundedWString(BoundedWString),
    Array(Array),
    Sequence(Sequence),
    BoundedSequence(BoundedSequence),
}

#[derive(Debug, Clone)]
pub enum ConstantType {
    BasicType(BasicType),
    String(String),
    WString(WString),
}
