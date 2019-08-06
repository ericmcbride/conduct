use nom::{sequence::tuple, IResult};

#[derive(Debug)]
pub struct ScenarioOutline {
    scenario: String,
    tags: Vec<String>,
    feature: String,
    given: String,
    when: String,
    then: String,
}

named!(scenario_outline_parser<&str, &str>,
    delimited!(tag!("Scenario Outline:"), take_until!("\n"), char!('\n'))
);

named!(tag_feature_parser<&str, (Vec<&str>, &str)>,
    alt!(
        many_till!(
            delimited!(tag!("@"), take_until!("\n"), char!('\n')),
            delimited!(tag!("Feature:"), take_until!("\n"), char!('\n'))
    ) | many_till!(
            delimited!(tag!("Feature:"), take_until!("\n"), char!('\n')),
            tag!("\n")
        )
    )
);

named!(given_parser<&str, &str>,
    delimited!(tag!("Given"), take_until!("\n"), char!('\n'))
);

named!(when_parser<&str, &str>,
    delimited!(tag!("When"), is_not!("\n"), char!('\n'))
);

named!(then_parser<&str, &str>,
    delimited!(alt!(tag!("Then") | tag!("And")), take_until!("\n"), char!('\n'))
);

/// Note: Not working with whitespace right now.  You have to trim.  Example code of it working:
/// ```
/// use std::fs::File;
/// use std::io::{BufRead, BufReader, Error, ErrorKind};
/// use std::path::Path;
/// use conduct::parse::build_outline;
///
/// let filename = "src/foo.feature";
/// let file = File::open(filename).unwrap();
/// let buffered = BufReader::new(file);
/// let lines = buffered.lines();
///
/// let mut file_str = "".to_string();
/// for l in lines {
///     file_str = file_str +  l.unwrap().trim() + "\n"
/// }
/// let scenario = build_outline(&file_str).unwrap();
/// println!("Scenario {:?}", scenario);
///```
pub fn build_outline(i: &str) -> IResult<&str, ScenarioOutline> {
    let (input, (tags_feature, scenario, given, when, then)) = tuple((
        tag_feature_parser,
        scenario_outline_parser,
        given_parser,
        when_parser,
        then_parser,
    ))(i)?;

    Ok((
        input,
        ScenarioOutline {
            scenario: scenario.trim().to_string(),
            tags: tags_feature.0.iter().map(|&x| x.to_string()).collect(),
            feature: tags_feature.1.trim().to_string(),
            given: given.trim().to_string(),
            when: when.trim().to_string(),
            then: then.trim().to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::build_outline;
    use super::given_parser;
    use super::scenario_outline_parser;
    use super::tag_feature_parser;
    use super::then_parser;
    use super::when_parser;
    use super::ScenarioOutline;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, ErrorKind};
    use std::path::Path;

    #[test]
    fn test_tag_feature_parser_with_tags() {
        let mut test_vec = Vec::new();
        test_vec.push("foo");
        test_vec.push("bar");

        assert_eq!(
            tag_feature_parser("@foo\n@bar\nFeature: This is my test feature\n").unwrap(),
            ("", (test_vec, " This is my test feature")),
        )
    }

    #[test]
    fn test_tag_feature_parser_without_tags() {
        assert_eq!(
            tag_feature_parser("Feature: This is my test feature\n").unwrap(),
            ("", (Vec::new(), " This is my test feature"))
        )
    }

    #[test]
    fn test_scenario_outline_parser() {
        assert_eq!(
            scenario_outline_parser("Scenario Outline: This is my outline\n").unwrap(),
            ("", " This is my outline")
        )
    }

    #[test]
    fn test_given_parser() {
        assert_eq!(
            given_parser("Given I have a task\n").unwrap(),
            ("", " I have a task")
        )
    }

    #[test]
    fn test_when_parser() {
        assert_eq!(
            when_parser("When I click a button\n").unwrap(),
            ("", " I click a button")
        )
    }

    #[test]
    fn test_then_parser() {
        assert_eq!(
            then_parser("Then i do some stuff\n").unwrap(),
            ("", " i do some stuff")
        )
    }

    #[test]
    fn test_then_with_and_parser() {
        assert_eq!(
            then_parser("And i do more stuff\n").unwrap(),
            ("", " i do more stuff")
        )
    }

    #[test]
    fn test_build_scenario_outline() {
        let filename = "src/foo.feature";
        let file = File::open(filename).unwrap();
        let buffered = BufReader::new(file);
        let lines = buffered.lines();
        let mut file_str = "".to_string();

        for l in lines {
            file_str = file_str + l.unwrap().trim() + "\n"
        }

        let mut expected_vec = Vec::new();
        expected_vec.push("smoke");
        expected_vec.push("wip");
        let str_vec = expected_vec.iter().map(|&x| x.to_string()).collect();

        let got = build_outline(&file_str).unwrap();
        let expected = ScenarioOutline {
            feature: "Fake Test Stuff".to_string(),
            given: "I have a login".to_string(),
            scenario: "This is a fake scenario".to_string(),
            tags: str_vec,
            when: "I type in a password and click login".to_string(),
            then: "the page redirects".to_string(),
        };

        assert_eq!(got.0, "");
        assert_eq!(got.1.feature, expected.feature);
        assert_eq!(got.1.given, expected.given);
        assert_eq!(got.1.scenario, expected.scenario);
        assert_eq!(got.1.tags, expected.tags);
        assert_eq!(got.1.when, expected.when);
        assert_eq!(got.1.then, expected.then);
    }
}
