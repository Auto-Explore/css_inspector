use crate::UrlDecodePlusError;

pub fn url_decode_plus(input: &str) -> Result<String, UrlDecodePlusError> {
    let bytes = input.as_bytes();
    let Some(mut i) = bytes.iter().position(|&b| matches!(b, b'+' | b'%')) else {
        return Ok(input.to_owned());
    };

    let mut out = Vec::with_capacity(bytes.len());
    out.extend_from_slice(&bytes[..i]);
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hi = from_hex(bytes[i + 1])?;
                let lo = from_hex(bytes[i + 2])?;
                out.push((hi << 4) | lo);
                i += 3;
            }
            b'%' => return Err(UrlDecodePlusError::IncompletePercentEscape),
            b => {
                out.push(b);
                i += 1;
            }
        }
    }

    Ok(String::from_utf8(out)?)
}

fn from_hex(b: u8) -> Result<u8, UrlDecodePlusError> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(UrlDecodePlusError::InvalidHexDigit(b)),
    }
}

#[cfg(test)]
mod url_decode_plus_tests {
    use super::url_decode_plus;

    #[test]
    fn returns_input_when_no_escapes() {
        assert_eq!(url_decode_plus("abcDEF123-_.*").unwrap(), "abcDEF123-_.*");
    }

    #[test]
    fn decodes_plus_and_percent_escapes() {
        assert_eq!(url_decode_plus("a+b").unwrap(), "a b");
        assert_eq!(url_decode_plus("abc%2Fdef").unwrap(), "abc/def");
    }

    #[test]
    fn rejects_incomplete_percent_escapes() {
        let err = url_decode_plus("%").unwrap_err();
        assert!(err.to_string().contains("incomplete percent-escape"));
        let err = url_decode_plus("%A").unwrap_err();
        assert!(err.to_string().contains("incomplete percent-escape"));
    }

    #[test]
    fn rejects_invalid_hex_digits() {
        let err = url_decode_plus("%GG").unwrap_err();
        assert!(err.to_string().contains("invalid hex digit"));
    }

    #[test]
    fn rejects_decoded_invalid_utf8() {
        let err = url_decode_plus("%FF").unwrap_err();
        assert!(err.to_string().contains("invalid utf-8 in decoded string"));
    }
}
