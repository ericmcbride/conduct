#[derive(Debug)]
pub struct ScenarioOutline {
    scenario: String,
    tags: String,
    when: String,
    then: String,
}

named!(scenario_outline_parser<&str, &str>,
    tag_s!("Scenario Outline:")
);

named!(tags_parser<&str, &str>,
    tag_s!("@")
);

named!(when_parser<&str, &str>,
    tag_s!("When")
);

named!(then_parser<&str, &str>,
    alt!(tag_s!("Then") | tag_s!("And"))
);

named!(parameter_parser<&str, &str>,
    delimited!(
        char!('<'),
        is_not!(">"),
        char!('>')
    )
);

named!(
    build_outline <&str,ScenarioOutline>,do_parse!(
        scenario_name: scenario_outline_parser >>
        tags: tags_parser >>
        when: when_parser >>
        then: then_parser >>
        (ScenarioOutline {
            scenario: scenario_name.to_string(),
            tags: tags.to_string(),
            when: when.to_string(),
            then: then.to_string()
        })
    )
);

#[cfg(test)]
mod tests {
    use super::tags_parser;
    use super::scenario_outline_parser;
    use super::when_parser;
    use super::then_parser;
    use super::parameter_parser;


    #[test]
    fn test_tag_parser() {
        assert_eq!(
            tags_parser("@foo"),
            Ok(("foo", "@"))
        )
    }

    #[test]
    fn test_tag_parser_remaining_values() {
        assert_eq!(tags_parser("@foo").unwrap().0, "foo")
    }

    #[test]
    fn test_scenario_outline_parser() {
        assert_eq!(
            scenario_outline_parser("Scenario Outline: This is a test scenario"),
            Ok((" This is a test scenario", "Scenario Outline:"))
        )
    }

    #[test]
    fn test_scenario_outline_parser_remaining_values() {
        assert_eq!(
            scenario_outline_parser("Scenario Outline: This is a test scenario").unwrap().0,
            " This is a test scenario"
        )
    }

    #[test]
    fn test_when_parser() {
        assert_eq!(
            when_parser("When I do a funky step"),
            Ok((" I do a funky step", "When"))
        )
    }

    #[test]
    fn test_when_parser_remaining_values() {
        assert_eq!(
            when_parser("When I do a funky step").unwrap().0,
            " I do a funky step"
        )
    }

    #[test]
    fn test_then_parser_with_then() {
        assert_eq!(
            then_parser("Then I am like wtf"),
            Ok((" I am like wtf", "Then"))
        )
    }

    #[test]
    fn test_then_parser_with_then_with_remaining_values() {
        assert_eq!(
            then_parser("Then I am like wtf").unwrap().0,
            " I am like wtf"
        )
    }

    #[test]
    fn test_then_parser_with_and() {
        assert_eq!(
            then_parser("And I look at myself"),
            Ok((" I look at myself", "And"))
        )
    }

    #[test]
    fn test_then_parser_with_and_remaining_values() {
        assert_eq!(
            then_parser("And I look at myself").unwrap().0,
            " I look at myself"
        )
    }

    #[test]
    fn test_parameter_parser() {
        assert_eq!(
            parameter_parser("<spaghetti.jpeg>"),
            Ok(("", "spaghetti.jpeg"))
        )
    }

    #[test]
    fn test_parameter_parser_remaining_values() {
        assert_eq!(
            parameter_parser("<spaghetti.jpeg>").unwrap().1,
            "spaghetti.jpeg"
        )
    }
}


