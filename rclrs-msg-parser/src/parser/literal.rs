use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while};
use nom::character::complete::{digit1, hex_digit1, oct_digit1, one_of};
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[inline]
fn flag_if_exist(i: &str) -> IResult<&str, String> {
    map(opt(map(one_of("+-"), |c| c.to_string())), |flag| {
        flag.unwrap_or_default()
    })(i)
}

pub fn integer_literal(i: &str) -> IResult<&str, anyhow::Result<i128>> {
    alt((bin_literal, oct_literal, hex_literal, dec_literal))(i)
}

pub fn bin_literal(i: &str) -> IResult<&str, anyhow::Result<i128>> {
    map(
        tuple((
            flag_if_exist,
            tag_no_case("0b"),
            separated_list1(tag("_"), take_while(|c| c == '0' || c == '1')),
        )),
        |(flag, _, digits)| {
            let out = i128::from_str_radix(&format!("{}{}", flag, digits.join("")), 2)?;
            Ok(out)
        },
    )(i)
}

pub fn oct_literal(i: &str) -> IResult<&str, anyhow::Result<i128>> {
    map(
        tuple((
            flag_if_exist,
            tag_no_case("0o"),
            separated_list1(tag("_"), oct_digit1),
        )),
        |(flag, _, digits)| {
            let out = i128::from_str_radix(&format!("{}{}", flag, digits.join("")), 8)?;
            Ok(out)
        },
    )(i)
}

pub fn dec_literal(i: &str) -> IResult<&str, anyhow::Result<i128>> {
    map(
        tuple((flag_if_exist, separated_list1(tag("_"), digit1))),
        |(flag, digits)| {
            let out = i128::from_str_radix(&format!("{}{}", flag, digits.join("")), 10)?;
            Ok(out)
        },
    )(i)
}

pub fn hex_literal(i: &str) -> IResult<&str, anyhow::Result<i128>> {
    map(
        tuple((
            flag_if_exist,
            tag_no_case("0x"),
            separated_list1(tag("_"), hex_digit1),
        )),
        |(flag, _, digits)| {
            let out = i128::from_str_radix(&format!("{}{}", flag, digits.join("")), 16)?;
            Ok(out)
        },
    )(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_flag() {
        assert_eq!(flag_if_exist("+").unwrap().1, "+".to_string());
        assert_eq!(flag_if_exist("-").unwrap().1, "-".to_string());
        assert_eq!(flag_if_exist("x").unwrap().1, "".to_string());
    }

    #[test]
    fn parse_integer_literal() {
        assert_eq!(integer_literal("101_010").unwrap().1.unwrap(), 101010);
    }

    #[test]
    fn parse_bin_literal() {
        assert_eq!(bin_literal("0b101_010").unwrap().1.unwrap(), 0b101010);
        assert_eq!(bin_literal("+0b101_010").unwrap().1.unwrap(), 0b101010);
        assert_eq!(bin_literal("-0b101_010").unwrap().1.unwrap(), -0b101010);
    }

    #[test]
    fn parse_oct_literal() {
        assert_eq!(oct_literal("0o12_345_670").unwrap().1.unwrap(), 0o12345670);
        assert_eq!(oct_literal("+0o12_345_670").unwrap().1.unwrap(), 0o12345670);
        assert_eq!(
            oct_literal("-0o12_345_670").unwrap().1.unwrap(),
            -0o12345670
        );
    }

    #[test]
    fn parse_dec_literal() {
        assert_eq!(dec_literal("123_456_789").unwrap().1.unwrap(), 123456789);
        assert_eq!(dec_literal("+123_456_789").unwrap().1.unwrap(), 123456789);
        assert_eq!(dec_literal("-123_456_789").unwrap().1.unwrap(), -123456789);
    }

    #[test]
    fn parse_hex_literal() {
        assert_eq!(hex_literal("0x789_aBc").unwrap().1.unwrap(), 0x789abc);
        assert_eq!(hex_literal("+0x789_aBc").unwrap().1.unwrap(), 0x789abc);
        assert_eq!(hex_literal("-0x789_aBc").unwrap().1.unwrap(), -0x789abc);
    }
}
