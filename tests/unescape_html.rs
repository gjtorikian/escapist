#[test]
fn unescape_html_without_semicolons() {
    let entities = "&#0000106&#0000097&#0000118&#0000097&#0000115&#0000099&#0000114&#0000105&#0000112&#0000116&#0000058;";
    let result = escapist::unescape_html(entities.as_bytes());
    assert_eq!("javascript:", std::str::from_utf8(&result).unwrap());
}
