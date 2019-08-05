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
/// let filename = "foo.feature";
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
fn build_outline(i: &str) -> IResult<&str, ScenarioOutline> {
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
            feature: tags_feature.1.trim().to_string(), //tags_feature.0,
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

    #[test]
    fn test_tag_parser() {
        let mut test_vec = Vec::new();
        test_vec.push("foo");
        test_vec.push("bar");

        assert_eq!(
            tag_feature_parser("@foo\n@bar\nFeature: This is my test feature\n").unwrap(),
            ("", (test_vec, " This is my test feature")),
        )
    }
}
