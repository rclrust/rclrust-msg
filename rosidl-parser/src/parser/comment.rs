use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::value;
use nom::sequence::{pair, tuple};
use nom::IResult;

pub fn single_line_comment(i: &str) -> IResult<&str, ()> {
    value((), pair(tag("//"), is_not("\n\r")))(i)
}

pub fn multi_line_comment(i: &str) -> IResult<&str, ()> {
    value((), tuple((tag("/*"), take_until("*/"), tag("*/"))))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_line_comment() {
        let (remain, _) = single_line_comment("// abc\ndef").unwrap();
        assert_eq!(remain, "\ndef");
    }

    #[test]
    fn test_single_line_comment_fail() {
        assert!(single_line_comment("/ abc\ndef").is_err());
    }

    #[test]
    fn test_multi_line_comment() {
        let (remain, _) = multi_line_comment("/* abc\n\nbcd\r */def").unwrap();
        assert_eq!(remain, "def");
    }

    #[test]
    fn test_multi_line_comment_start_only() {
        assert!(multi_line_comment("/* abc").is_err());
    }
}
