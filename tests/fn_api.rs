use readable_regex::ReadableRe::*;
use readable_regex::*;

#[test]
fn american_phone_numbers() {
    let rre = concat([
        digit(),
        digit(),
        digit(),
        raw_regex("-"),
        digit(),
        digit(),
        digit(),
        raw_regex("-"),
        digit(),
        digit(),
        digit(),
        digit(),
    ]);
    assert_eq!(rre.to_string(), r"\d\d\d-\d\d\d-\d\d\d\d");

    let rre = concat([
        exactly(3, digit()),
        raw_regex("-"),
        exactly(3, digit()),
        raw_regex("-"),
        exactly(4, digit()),
    ]);

    assert_eq!(rre.to_string(), r"\d{3}-\d{3}-\d{4}");

    let rre = concat([
        group(exactly(3, Digit)),
        raw_regex("-"),
        group(concat([
            exactly(3, Digit),
            raw_regex("-"),
            exactly(4, Digit),
        ])),
    ]);

    assert_eq!(rre.to_string(), r"(\d{3})-(\d{3}-\d{4})")
}

#[test]
fn simple_matches() {
    assert_eq!(
        concat([Raw("First Name: "), group(Anything)]).to_string(),
        "First Name: (.*?)"
    );

    assert_eq!(
        concat([Raw("First Name: "), group(Everything)]).to_string(),
        "First Name: (.*)"
    );

    assert_eq!(zero_or_more(Raw("x")).to_string(), "x*");
    assert_eq!(one_or_more(Raw("x")).to_string(), "x+");

    assert_eq!(ranged(3..5, Raw("x")).to_string(), "x{3,5}");
    assert_eq!(at_least(2, Raw("x")).to_string(), "x{2,}");
    assert_eq!(at_most(2, Raw("x")).to_string(), "x{,2}");
}

#[test]
fn big_ass_query() {
    let query: &str =
        r"((\d{3}|\(\d{3}\))?(\s|-|\.)?\d{3}(\s|-|\.)\d{4}(\s*(ext|x|ext.)\s*\d{2,5})?)";

    let re = group(concat([
        optional(group(either([
            exactly(3, Digit),
            concat([OpenParenthesis, exactly(3, Digit), CloseParenthesis]),
        ]))),
        optional(group(either([Whitespace, Raw("-"), Period]))),
        exactly(3, Digit),
        group(either([Whitespace, Raw("-"), Period])),
        exactly(4, Digit),
        optional(group(concat([
            zero_or_more(Whitespace),
            group(either([Raw("ext"), Raw("x"), Raw("ext.")])),
            zero_or_more(Whitespace),
            ranged(2..5, Digit),
        ]))),
    ]));
    assert_eq!(re.to_string(), query);
}

#[test]
fn some_chars() {
    let query = "[YZ][BCE-HMO-Y][BEFN][A-Z][0-9][0-9]_KWBC_[0-9]{6}";
    let re = chars("YZ".chars())
        + chars("BCE-HMO-Y".chars())
        + chars("BEFN".chars())
        + chars("A-Z".chars())
        + chars("0-9".chars())
        + chars("0-9".chars())
        + Raw("_KWBC_")
        + exactly(6, chars("0-9".chars()));
    assert_eq!(re.to_string(), query);
}

#[test]
fn check_sum_match() {
    let printable = r#"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!"\#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c"#;
    let query = format!("Check[ ]?sum[ ]+is[ ]+([{}])", printable);
    let re: ReadableRe = Raw("Check")
        + optional(chars(" ".chars()))
        + "sum".into()
        + one_or_more(chars(" ".chars()))
        + "is".into()
        + one_or_more(chars(" ".chars()))
        + group(chars(printable.chars()));

    assert_eq!(re.to_string(), query);
}

#[test]
fn test_concat() {
    assert_eq!(
        concat(["dog", "cat", "moose"].map(Into::into)).to_string(),
        "dogcatmoose"
    );

    assert_eq!(concat([]).to_string(), "");
}

#[test]
fn test_escape() {
    let escaped = regex::escape("hello");
    let escaped_re = escape("hello".into());
    assert_eq!(escaped_re.to_string(), "hello");
    assert_eq!(escaped_re.to_string(), escaped);

    for char in ".^$*+?{}[]\\|()".chars().map(|c| c.to_string()) {
        assert_eq!(regex::escape(&char), escape(char.into()).to_string());
    }
}

#[test]
fn test_group() {
    assert_eq!(group("cat".into()).to_string(), "(cat)");
    assert_eq!(
        group(concat(["cat", "dog", "moose"].map(Into::into))).to_string(),
        "(catdogmoose)"
    );
    assert_eq!(
        group(concat([
            "cat".into(),
            group(concat(["dog", "moose"].map(Into::into)))
        ]))
        .to_string(),
        "(cat(dogmoose))"
    );
    assert_eq!(
        group(concat([
            "cat".into(),
            group(concat(["dog".into(), group("moose".into())]))
        ]))
        .to_string(),
        "(cat(dog(moose)))"
    );
}

#[test]
fn test_positive_look_ahead() {
    assert_eq!(positive_look_ahead("cat".into()).to_string(), "(?=cat)");
}

#[test]
fn test_negative_look_ahead() {
    assert_eq!(negative_look_ahead("cat".into()).to_string(), "(?!cat)");
}

#[test]
fn test_positive_look_behind() {
    assert_eq!(positive_look_behind("cat".into()).to_string(), "(?<=cat)");
}

#[test]
fn test_negative_look_behind() {
    assert_eq!(negative_look_behind("cat".into()).to_string(), "(?<!cat)");
}

#[test]
fn test_named_group() {
    assert_eq!(named_group("foo", "cat".into()).to_string(), "(?P<foo>cat)");
}

#[test]
fn test_nocap_group() {
    assert_eq!(non_capture_group("cat".into()).to_string(), "(?:cat)");
}

#[test]
fn test_optional() {
    assert_eq!(optional("cat".into()).to_string(), "cat?");
}

#[test]
fn test_either() {
    assert_eq!(
        either(["cat", "dog", "moose"].map(Into::into)).to_string(),
        "cat|dog|moose"
    );
}

#[test]
fn test_exactly() {
    assert_eq!(exactly(10, "cat".into()).to_string(), "cat{10}");
}

#[test]
fn test_ranged() {
    assert_eq!(ranged(1..10, "cat".into()).to_string(), "cat{1,10}");
}

#[test]
fn test_at_least() {
    assert_eq!(at_least(1, "cat".into()).to_string(), "cat{1,}");
}

#[test]
fn test_at_most() {
    assert_eq!(at_most(1, "cat".into()).to_string(), "cat{,1}");
}

#[test]
fn test_zero_or_more() {
    assert_eq!(zero_or_more("x".into()).to_string(), "x*");
}

#[test]
fn test_zero_or_more_lazy() {
    assert_eq!(zero_or_more_lazy("x".into()).to_string(), "x*?");
}

#[test]
fn test_one_or_more() {
    assert_eq!(one_or_more("x".into()).to_string(), "x+");
}

#[test]
fn test_one_or_more_lazy() {
    assert_eq!(one_or_more_lazy("x".into()).to_string(), "x+?");
}

#[test]
fn test_starts_with() {
    assert_eq!(starts_with("cat".into()).to_string(), "^cat");
}

#[test]
fn test_ends_with() {
    assert_eq!(ends_with("cat".into()).to_string(), "cat$");
}

#[test]
fn test_starts_and_ends_with() {
    assert_eq!(starts_and_ends_with("cat".into()).to_string(), "^cat$");
}

#[test]
fn test_chars() {
    assert_eq!(chars("A-Z".chars()).to_string(), "[A-Z]");
    assert_eq!(chars(['c', 'a', 't']).to_string(), "[cat]");
}

#[test]
fn test_not_chars() {
    assert_eq!(not_chars("A-Z".chars()).to_string(), "[^A-Z]");
    assert_eq!(not_chars(['c', 'a', 't']).to_string(), "[^cat]");
}

#[test]
fn test_hexadecimal() {
    let re = hexadecimal().compile().unwrap();
    // match A-F, a-f, 0-9
    for c in (65..71).chain(97..103).chain(48..58) {
        assert!(re.is_match(&char::from(c).to_string()));
    }
    // no match A-F, a-f, 0-9
    for c in (71..91).chain(103..123).chain(192..255).chain(33..48) {
        assert!(!re.is_match(&char::from(c).to_string()));
    }
}

#[test]
fn test_non_hexadecimal() {
    let re = non_hexadecimal().compile().unwrap();
    // match A-F, a-f, 0-9
    for c in (65..71).chain(97..103).chain(48..58) {
        assert!(!re.is_match(&char::from(c).to_string()));
    }
    // no match A-F, a-f, 0-9
    for c in (71..91).chain(103..123).chain(192..255).chain(33..48) {
        assert!(re.is_match(&char::from(c).to_string()));
    }
}

#[test]
fn test_ascii_letter() {
    let letter = ascii_letter().compile().unwrap();
    for c in (0u8..255).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if c.is_ascii_alphabetic() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}

#[test]
fn test_ascii_lowercase() {
    let letter = ascii_lowercase().compile().unwrap();
    for c in (0u8..255).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if c.is_ascii_lowercase() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}

#[test]
fn test_ascii_not_lowercase() {
    let letter = ascii_non_lowercase().compile().unwrap();
    for c in (0u8..255).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if !c.is_ascii_lowercase() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}

#[test]
fn test_ascii_uppercase() {
    let letter = ascii_uppercase().compile().unwrap();
    for c in (0u8..255).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if c.is_ascii_uppercase() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}

#[test]
fn test_not_ascii_uppercase() {
    let letter = ascii_non_uppercase().compile().unwrap();
    for c in (0u8..255).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if !c.is_ascii_uppercase() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}

#[test]
fn test_ascii_alphanumeric() {
    let letter = ascii_alphanumeric().compile().unwrap();
    for c in (0u8..255).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if c.is_ascii_alphanumeric() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}

#[test]
fn test_not_ascii_alphanumeric() {
    let letter = ascii_non_alphanumeric().compile().unwrap();
    for c in (0u8..255).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if !c.is_ascii_alphanumeric() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}

#[test]
fn test_ascii_numeric() {
    let letter = ascii_numeric().compile().unwrap();
    for c in (0u8..128).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if c.is_numeric() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}

#[test]
fn test_not_ascii_numeric() {
    let letter = ascii_non_numeric().compile().unwrap();
    for c in (0u8..128).map(char::from) {
        let res = letter.is_match(&c.to_string());
        if !c.is_numeric() {
            assert!(res);
        } else {
            assert!(!res);
        }
    }
}
