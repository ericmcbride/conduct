use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{delimited, preceded};
use nom::IResult;

#[derive(Debug)]
pub struct ScenarioOutline {
    scenario: String,
    tags: Vec<String>,
    feature: String,
    given: String,
    when: String,
    then: String,
}

fn ws(i: &str) -> IResult<&str, &str> {
    tag("\n")(i)
}

fn until_n(i: &str) -> IResult<&str, &str> {
    take_until("\n")(i)
}

fn language_parser(i: &str) -> IResult<&str, Option<&str>> {
    opt(delimited(tag("# language:"), until_n, ws))(i)
}

fn tag_parser(i: &str) -> IResult<&str, Option<Vec<&str>>> {
    opt(many1(delimited(tag("@"), until_n, ws)))(i)
}

fn comment_parser(i: &str) -> IResult<&str, Option<&str>> {
    opt(delimited(tag("#"), until_n, ws))(i)
}

fn feature_parser(i: &str) -> IResult<&str, &str> {
    delimited(tag("Feature:"), until_n, ws)(i)
}

fn scenario_parser(i: &str) -> IResult<&str, &str> {
    delimited(tag("Scenario Outline:"), until_n, ws)(i)
}

fn given_parser(i: &str) -> IResult<&str, Vec<&str>> {
    many1(delimited(tag("Given"), until_n, ws))(i)
}

fn when_parser(i: &str) -> IResult<&str, Vec<&str>> {
    many1(delimited(tag("When"), until_n, ws))(i)
}

fn then_parser(i: &str) -> IResult<&str, Vec<&str>> {
    many1(delimited(tag("Then"), until_n, ws))(i)
}

fn and_parser(i: &str) -> IResult<&str, Vec<&str>> {
    many1(delimited(tag("And"), until_n, ws))(i)
}

fn then_and_parser(i: &str) -> IResult<&str, Vec<&str>> {
    alt((then_parser, and_parser))(i)
}

#[cfg(test)]
mod tests {
    use super::and_parser;
    use super::comment_parser;
    use super::feature_parser;
    use super::scenario_parser;
    use super::given_parser;
    use super::language_parser;
    use super::tag_parser;
    use super::then_parser;
    use super::until_n;
    use super::when_parser;
    use super::ws;


    #[test]
    fn test_language_parser() {
        assert_eq!(
            language_parser("# language: en\n").unwrap(),
            ("", (Some(" en")))
        )
    }

    #[test]
    fn test_comment_parser() {
        assert_eq!(
            comment_parser("# this is a comment\n").unwrap(),
            ("", Some(" this is a comment"))
        )
    }

    #[test]
    fn test_tag_parser() {
        let mut test_vec = Vec::new();
        test_vec.push("foo");
        test_vec.push("bar");

        assert_eq!(
            tag_parser("@foo\n@bar\nFeature: This is my test feature\n").unwrap(),
            ("Feature: This is my test feature\n", Some(test_vec)),
        )
    }

    #[test]
    fn test_feature_parser() {
        let mut test_vec = Vec::new();
        test_vec.push(" This is my test feature");
        assert_eq!(
            feature_parser("Feature: This is my test feature\n").unwrap(),
            ("", " This is my test feature")
        )
    }

    #[test]
    fn test_scenario_parser() {
        assert_eq!(
            scenario_parser("Scenario Outline: This is my outline\n").unwrap(),
            ("", " This is my outline")
        )
    }

    #[test]
    fn test_given_parser() {
        let mut test_vec = Vec::new();
        test_vec.push(" I have a task");
        assert_eq!(
            given_parser("Given I have a task\n").unwrap(),
            ("", test_vec)
        )
    }

    #[test]
    fn test_when_parser() {
        let mut test_vec = Vec::new();
        test_vec.push(" I click a button");
        assert_eq!(
            when_parser("When I click a button\n").unwrap(),
            ("", test_vec)
        )
    }

    #[test]
    fn test_then_parser() {
        let mut test_vec = Vec::new();
        test_vec.push(" i do some stuff");
        assert_eq!(
            then_parser("Then i do some stuff\n").unwrap(),
            ("", test_vec)
        )
    }

    #[test]
    fn test_and_parser() {
        let mut test_vec = Vec::new();
        test_vec.push(" i do more stuff");
        assert_eq!(
            and_parser("And i do more stuff\n").unwrap(),
            ("", test_vec)
        )
    }
}
