
#[derive(Default)]
#[allow(dead_code)]
pub struct Errors {
    punycode: bool,
    // check_hyphens: bool,
    // check_bidi: bool,
    // start_combining_mark: bool,
    // invalid_mapping: bool,
    // nfc: bool,
    // disallowed_by_std3_ascii_rules: bool,
    // disallowed_mapped_in_std3: bool,
    // disallowed_character: bool,
    // too_long_for_dns: bool,
    // too_short_for_dns: bool,
    // disallowed_in_idna_2008: bool,
}

pub fn domain_to_ascii(domain: &str) -> Result<String, Errors> {
    // without idna feature, we can't verify that xn-- domains correctness
    let domain = domain.to_lowercase();
    if domain.is_ascii() && domain.split('.').all(|s| !s.starts_with("xn--")) {
        Ok(domain)
    } else {
        Err(Errors{punycode: true, ..Default::default()})
    }
}

/// The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm,
/// with the `beStrict` flag set.
pub fn domain_to_ascii_strict(domain: &str) -> Result<String, Errors> {
    domain_to_ascii(domain)
}

/// The [domain to Unicode](https://url.spec.whatwg.org/#concept-domain-to-unicode) algorithm.
///
/// Return the Unicode representation of a domain name,
/// normalizing characters (upper-case to lower-case and other kinds of equivalence)
/// and decoding Punycode as necessary.
///
/// This may indicate [syntax violations](https://url.spec.whatwg.org/#syntax-violation)
/// but always returns a string for the mapped domain.
pub fn domain_to_unicode(domain: &str) -> (String, Result<(), Errors>) {
    (domain.to_string(), Ok(()))
}
