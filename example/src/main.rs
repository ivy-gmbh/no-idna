use url;

fn main() {
    let url = url::Url::parse("http://example.com/").unwrap();
    assert_eq!(url.to_string(), "http://example.com/");

    // This URL contains unicode characters, so parsing it will fail
    let url = url::Url::parse("http://goșu.ro/");
    assert!(url.is_err());

    // This URL is in IDNA form, so parsing it will fail as the
    // url crate cannot perform normalization
    let url = url::Url::parse("http://xn--gou-2lb.ro/");
    assert!(url.is_err());
}