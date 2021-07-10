use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::number::complete::recognize_float;
use nom::sequence::tuple;
use nom::IResult;

use crate::field_type::{BasicType, ConstantType};
use crate::parser::identifier::constant_name;
use crate::parser::literal::{bool_literal, integer_literal};
use crate::spec::Constant;

pub fn integer_const_def(i: &str) -> IResult<&str, anyhow::Result<Constant>> {
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
            constant_name,
            space0,
            tag("="),
            space0,
            integer_literal,
        )),
        |(basic_type, _, name, _, _, _, value)| {
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
            let value = value?;
            basic_type.check_integer_range(value)?;

            Ok(Constant {
                name: name.to_string(),
                r#type: ConstantType::BasicType(basic_type),
                value: value.to_string(),
            })
        },
    )(i)
}

pub fn float_const_def(i: &str) -> IResult<&str, Constant> {
    map(
        tuple((
            alt((tag("float32"), tag("float64"))),
            space1,
            constant_name,
            space0,
            tag("="),
            space0,
            recognize_float,
        )),
        |(basic_type, _, name, _, _, _, value)| {
            let basic_type = match basic_type {
                "float32" => BasicType::F32,
                "float64" => BasicType::F64,
                _ => unreachable!(),
            };
            Constant {
                name: name.to_string(),
                r#type: ConstantType::BasicType(basic_type),
                value: value.to_string(),
            }
        },
    )(i)
}

pub fn bool_const_def(i: &str) -> IResult<&str, Constant> {
    map(
        tuple((
            tag("bool"),
            space1,
            constant_name,
            space0,
            tag("="),
            space0,
            bool_literal,
        )),
        |(_, _, name, _, _, _, value)| Constant {
            name: name.to_string(),
            r#type: ConstantType::BasicType(BasicType::Bool),
            value: value.to_string(),
        },
    )(i)
}

#[cfg(test)]
mod test {
    use super::*;

    use paste::paste;

    macro_rules! impl_int_const_def_test {
        ($test_common_name:ident, $msg_type:expr, $basic_type:expr, $out_range:expr) => {
            paste! {
                #[test]
                fn $test_common_name() {
                    let input = format!("{} AAA=10", $msg_type);
                    let result = integer_const_def(&input).unwrap().1.unwrap();
                    assert_eq!(result.name, "AAA");
                    assert_eq!(result.r#type, ConstantType::BasicType($basic_type));
                    assert_eq!(result.value, "10".to_string());
                }

                #[test]
                fn [<$test_common_name _without_value>]() {
                    let input = format!("{} AAA", $msg_type);
                    let result = integer_const_def(&input);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$test_common_name _out_range>]() {
                    let input = format!("{} AAA={}", $msg_type, $out_range);
                    let result = integer_const_def(&input).unwrap();
                    assert!(result.1.is_err());
                }
            }
        };
    }

    impl_int_const_def_test!(parse_u8_const_def, "uint8", BasicType::U8, 1u64 << 8);
    impl_int_const_def_test!(parse_u16_const_def, "uint16", BasicType::U16, 1u64 << 16);
    impl_int_const_def_test!(parse_u32_const_def, "uint32", BasicType::U32, 1u64 << 32);
    impl_int_const_def_test!(parse_u64_const_def, "uint64", BasicType::U64, 1u128 << 64);

    impl_int_const_def_test!(parse_i8_const_def, "int8", BasicType::I8, 1u64 << 7);
    impl_int_const_def_test!(parse_i16_const_def, "int16", BasicType::I16, 1u64 << 15);
    impl_int_const_def_test!(parse_i32_const_def, "int32", BasicType::I32, 1u64 << 31);
    impl_int_const_def_test!(parse_i64_const_def, "int64", BasicType::I64, 1u64 << 63);

    impl_int_const_def_test!(parse_char_const_def, "char", BasicType::Char, 1u64 << 8);
    impl_int_const_def_test!(parse_byte_const_def, "byte", BasicType::Byte, 1u64 << 8);

    #[test]
    fn parse_f32_const_def() {
        let result = float_const_def("float32 AAA=2.0").unwrap().1;
        assert_eq!(result.name, "AAA");
        assert_eq!(result.r#type, ConstantType::BasicType(BasicType::F32));
        assert_eq!(result.value, "2.0".to_string());
    }

    #[test]
    fn parse_f32_const_def_without_value() {
        let result = float_const_def("float32 AAA");
        assert!(result.is_err());
    }

    #[test]
    fn parse_f64_const_def() {
        let result = float_const_def("float64 AAA=2.0").unwrap().1;
        assert_eq!(result.name, "AAA");
        assert_eq!(result.r#type, ConstantType::BasicType(BasicType::F64));
        assert_eq!(result.value, "2.0".to_string());
    }

    #[test]
    fn parse_f64_const_def_with_default() {
        let result = float_const_def("float64 AAA");
        assert!(result.is_err());
    }

    #[test]
    fn parse_bool_const_def() {
        let result = bool_const_def("bool AAA=true").unwrap().1;
        assert_eq!(result.name, "AAA");
        assert_eq!(result.r#type, ConstantType::BasicType(BasicType::Bool));
        assert_eq!(result.value, "true".to_string());
    }

    #[test]
    fn parse_bool_const_def_with_default() {
        let result = bool_const_def("bool AAA");
        assert!(result.is_err());
    }
}
