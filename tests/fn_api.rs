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
