use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::{map, opt};
use nom::number::complete::recognize_float;
use nom::sequence::{preceded, tuple};
use nom::IResult;

use crate::field_type::{BasicType, FieldType};
use crate::parser::identifier::identifier;
use crate::parser::literal::{bool_literal, integer_literal};
use crate::spec::Field;

pub fn integer_field_def(i: &str) -> IResult<&str, anyhow::Result<Field>> {
    map(
        tuple((
            alt((
                tag("uint8"),
                tag("uint16"),
                tag("uint32"),
                tag("uint64"),
                tag("int8"),
                tag("int16"),
                tag("int32"),
                tag("int64"),
                tag("char"),
                tag("byte"),
            )),
            space1,
            identifier,
            opt(preceded(space1, integer_literal)),
        )),
        |(basic_type, _, name, default)| {
            let basic_type = match basic_type {
                "uint8" => BasicType::U8,
                "uint16" => BasicType::U16,
                "uint32" => BasicType::U32,
                "uint64" => BasicType::U64,
                "int8" => BasicType::I8,
                "int16" => BasicType::I16,
                "int32" => BasicType::I32,
                "int64" => BasicType::I64,
                "char" => BasicType::Char,
                "byte" => BasicType::Byte,
                _ => unreachable!(),
            };
            let default = match default {
                Some(Ok(v)) => {
                    basic_type.check_integer_range(v)?;
                    Ok(Some(v.to_string()))
                }
                Some(Err(e)) => Err(e),
                None => Ok(None),
            }?;

            Ok(Field {
                name: name.to_string(),
                r#type: FieldType::BasicType(basic_type),
                default,
            })
        },
    )(i)
}

pub fn float_field_def(i: &str) -> IResult<&str, Field> {
    map(
        tuple((
            alt((tag("float32"), tag("float64"))),
            space1,
            identifier,
            opt(preceded(space1, recognize_float)),
        )),
        |(basic_type, _, name, default)| {
            let basic_type = match basic_type {
                "float32" => BasicType::F32,
                "float64" => BasicType::F64,
                _ => unreachable!(),
            };
            Field {
                name: name.to_string(),
                r#type: FieldType::BasicType(basic_type),
                default: default.map(|s| s.to_string()),
            }
        },
    )(i)
}

pub fn bool_field_def(i: &str) -> IResult<&str, Field> {
    map(
        tuple((
            tag("bool"),
            space1,
            identifier,
            opt(preceded(space1, bool_literal)),
        )),
        |(_, _, name, default)| Field {
            name: name.to_string(),
            r#type: FieldType::BasicType(BasicType::Bool),
            default: default.map(|b| b.to_string()),
        },
    )(i)
}

#[cfg(test)]
mod test {
    use super::*;

    use paste::paste;

    macro_rules! impl_int_field_def_test {
        ($test_common_name:ident, $msg_type:expr, $basic_type:expr, $out_range:expr) => {
            paste! {
                #[test]
                fn [<$test_common_name unc>]() {
                    let input = format!("{} aaa", $msg_type);
                    let result = integer_field_def(&input).unwrap().1.unwrap();
                    assert_eq!(result.name, "aaa");
                    assert_eq!(result.r#type, FieldType::BasicType($basic_type));
                    assert_eq!(result.default, None);
                }

                #[test]
                fn [<$test_common_name _with_default>]() {
                    let input = format!("{} aaa 10", $msg_type);
                    let result = integer_field_def(&input).unwrap().1.unwrap();
                    assert_eq!(result.name, "aaa");
                    assert_eq!(result.r#type, FieldType::BasicType($basic_type));
                    assert_eq!(result.default, Some("10".to_string()));
                }

                #[test]
                fn [<$test_common_name _with_default_out_range>]() {
                    let input = format!("{} aaa {}", $msg_type, $out_range);
                    let result = integer_field_def(&input).unwrap();
                    assert!(result.1.is_err());
                }
            }
        };
    }

    impl_int_field_def_test!(parse_u8_field_def, "uint8", BasicType::U8, 1u64 << 8);
    impl_int_field_def_test!(parse_u16_field_def, "uint16", BasicType::U16, 1u64 << 16);
    impl_int_field_def_test!(parse_u32_field_def, "uint32", BasicType::U32, 1u64 << 32);
    impl_int_field_def_test!(parse_u64_field_def, "uint64", BasicType::U64, 1u128 << 64);

    impl_int_field_def_test!(parse_i8_field_def, "int8", BasicType::I8, 1u64 << 7);
    impl_int_field_def_test!(parse_i16_field_def, "int16", BasicType::I16, 1u64 << 15);
    impl_int_field_def_test!(parse_i32_field_def, "int32", BasicType::I32, 1u64 << 31);
    impl_int_field_def_test!(parse_i64_field_def, "int64", BasicType::I64, 1u128 << 63);

    impl_int_field_def_test!(parse_char_field_def, "char", BasicType::Char, 1u64 << 8);
    impl_int_field_def_test!(parse_byte_field_def, "byte", BasicType::Byte, 1u64 << 8);

    #[test]
    fn parse_f32_field_def() {
        let result = float_field_def("float32 aaa").unwrap().1;
        assert_eq!(result.name, "aaa");
        assert_eq!(result.r#type, FieldType::BasicType(BasicType::F32));
        assert_eq!(result.default, None);
    }

    #[test]
    fn parse_f32_field_def_with_default() {
        let result = float_field_def("float32 aaa 2.0").unwrap().1;
        assert_eq!(result.name, "aaa");
        assert_eq!(result.r#type, FieldType::BasicType(BasicType::F32));
        assert_eq!(result.default, Some("2.0".to_string()));
    }

    #[test]
    fn parse_f64_field_def() {
        let result = float_field_def("float64 aaa").unwrap().1;
        assert_eq!(result.name, "aaa");
        assert_eq!(result.r#type, FieldType::BasicType(BasicType::F64));
        assert_eq!(result.default, None);
    }

    #[test]
    fn parse_f64_field_def_with_default() {
        let result = float_field_def("float64 aaa 2.0").unwrap().1;
        assert_eq!(result.name, "aaa");
        assert_eq!(result.r#type, FieldType::BasicType(BasicType::F64));
        assert_eq!(result.default, Some("2.0".to_string()));
    }

    #[test]
    fn parse_bool_field_def() {
        let result = bool_field_def("bool aaa").unwrap().1;
        assert_eq!(result.name, "aaa");
        assert_eq!(result.r#type, FieldType::BasicType(BasicType::Bool));
        assert_eq!(result.default, None);
    }

    #[test]
    fn parse_bool_field_def_with_default() {
        let result = bool_field_def("bool aaa false").unwrap().1;
        assert_eq!(result.name, "aaa");
        assert_eq!(result.r#type, FieldType::BasicType(BasicType::Bool));
        assert_eq!(result.default, Some("false".to_string()));
    }
}
