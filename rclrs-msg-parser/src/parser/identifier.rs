use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric0, alphanumeric1, digit1, one_of};
use nom::combinator::{opt, recognize};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, tuple};
use nom::IResult;

fn upperalpha(i: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}

fn loweralpha(i: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(i)
}

fn numeric(i: &str) -> IResult<&str, char> {
    one_of("0123456789")(i)
}

pub fn package_name(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        loweralpha,
        opt(tag("_")),
        separated_list1(tag("_"), many1(alt((loweralpha, numeric)))),
    )))(i)
}

pub fn field_name(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        loweralpha,
        opt(tag("_")),
        separated_list1(tag("_"), many1(alt((loweralpha, numeric)))),
    )))(i)
}

pub fn message_name(i: &str) -> IResult<&str, &str> {
    recognize(pair(upperalpha, alphanumeric0))(i)
}

pub fn constant_name(i: &str) -> IResult<&str, &str> {
    recognize(separated_list1(tag("_"), many1(upperalpha)))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_field_name() {
        let (remain, actual) = field_name("abc034_fs3_u3 = 20").unwrap();
        assert_eq!(actual, "abc034_fs3_u3");
        assert_eq!(remain, " = 20");
    }

    #[test]
    fn parse_field_name_should_fail_if_starting_with_underscore() {
        assert!(field_name("_invalid_indentifier").is_err());
    }

    #[test]
    fn parse_field_name_should_fail_if_starting_with_number() {
        assert!(field_name("0invalid_indentifier").is_err());
    }

    #[test]
    fn parse_message_name() {
        assert_eq!(message_name("StdMsgs12").unwrap().1, "StdMsgs12");
    }

    #[test]
    fn parse_message_name_should_fail_if_starting_with_wrong_char() {
        assert!(message_name("aStdMsgs12").is_err());
    }

    #[test]
    fn parse_constant_name() {
        assert_eq!(constant_name("C_O_N_STAN_Ta").unwrap().1, "C_O_N_STAN_T");
    }

    #[test]
    fn parse_constant_name_should_fail_if_starting_with_underscore() {
        assert!(constant_name("_C_O_N_STAN_Ta").is_err());
    }
}
