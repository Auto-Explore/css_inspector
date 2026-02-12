use super::*;

#[test]
fn clip_rect_allows_whitespace_or_commas_between_four_args() {
    let config = Config::default();

    let report = validate_css_text("a{clip:rect(1  2 3 4)}", &config).unwrap();
    assert_eq!(report.errors, 0, "{:?}", report.messages);

    let report = validate_css_text("a{clip:rect(1, 2, 3, 4)}", &config).unwrap();
    assert_eq!(report.errors, 0, "{:?}", report.messages);
}

#[test]
fn clip_rect_rejects_too_many_args() {
    let config = Config::default();
    let report = validate_css_text("a{clip:rect(1, 2, 3, 4, 5)}", &config).unwrap();
    assert_eq!(report.errors, 1, "{:?}", report.messages);
}
