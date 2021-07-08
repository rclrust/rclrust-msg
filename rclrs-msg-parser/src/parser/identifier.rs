use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

pub fn identifier(i: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identifier() {
        let (remain, actual) = identifier("abc034_fs3___u3 = 20").unwrap();
        assert_eq!(actual, "abc034_fs3___u3");
        assert_eq!(remain, " = 20");
    }

    #[test]
    fn test_identifier_should_fail_if_starting_with_underscore() {
        assert!(identifier("_invalid_indentifier").is_err());
    }

    #[test]
    fn test_identifier_should_fail_if_starting_with_number() {
        assert!(identifier("0invalid_indentifier").is_err());
    }
}
