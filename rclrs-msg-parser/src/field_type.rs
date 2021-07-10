use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    // floating point type
    F32,
    F64,
    // long double is not supported

    // boolean type
    Bool,

    // duplicated type
    Char, // equal to U8
    Byte, // equal to U8
}

impl BasicType {
    pub fn check_integer_range(self, v: i128) -> anyhow::Result<()> {
        match self {
            Self::U8 | Self::Char | Self::Byte => {
                let _ = u8::try_from(v)?;
            }
            Self::U16 => {
                let _ = u16::try_from(v)?;
            }
            Self::U32 => {
                let _ = u32::try_from(v)?;
            }
            Self::U64 => {
                let _ = u64::try_from(v)?;
            }
            Self::I8 => {
                let _ = i8::try_from(v)?;
            }
            Self::I16 => {
                let _ = i16::try_from(v)?;
            }
            Self::I32 => {
                let _ = i32::try_from(v)?;
            }
            Self::I64 => {
                let _ = i64::try_from(v)?;
            }
            Self::F32 | Self::F64 | Self::Bool => {
                // ignore floating point type or boolean type
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedType(std::string::String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedString(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedWString(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NestableType {
    BasicType(BasicType),
    NamedType(NamedType),
    String(String),
    WString(WString),
    BoundedString(BoundedString),
    BoundedWString(BoundedWString),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
    pub value_type: NestableType,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    pub value_type: NestableType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedSequence {
    pub value_type: NestableType,
    pub max_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstantType {
    BasicType(BasicType),
    String(String),
    WString(WString),
}
