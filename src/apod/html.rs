/// Replaces the HTML entities WordPress emits with the characters they stand for. Unknown entities
/// are left as written so no text is ever lost.
pub fn decode_entities(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        match rest.find(';').filter(|end| *end <= 10) {
            Some(end) => match entity_char(&rest[1..end]) {
                Some(c) => {
                    out.push(c);
                    rest = &rest[end + 1..];
                }
                None => {
                    out.push('&');
                    rest = &rest[1..];
                }
            },
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// The character behind an entity name or numeric reference (the text between `&` and `;`).
fn entity_char(name: &str) -> Option<char> {
    if let Some(hex) = name.strip_prefix("#x").or_else(|| name.strip_prefix("#X")) {
        return u32::from_str_radix(hex, 16).ok().and_then(char::from_u32);
    }
    if let Some(dec) = name.strip_prefix('#') {
        return dec.parse::<u32>().ok().and_then(char::from_u32);
    }
    match name {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        "nbsp" => Some(' '),
        "ndash" => Some('–'),
        "mdash" => Some('—'),
        "lsquo" => Some('‘'),
        "rsquo" => Some('’'),
        "ldquo" => Some('“'),
        "rdquo" => Some('”'),
        "hellip" => Some('…'),
        _ => None,
    }
}

/// Collapses every run of whitespace, newlines and tabs included, into a single space.
pub fn collapse_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Visible text of an HTML fragment: tags dropped, line breaks and paragraph ends read as spaces,
/// entities decoded, whitespace collapsed.
pub fn strip_tags(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(open) = rest.find('<') {
        text.push_str(&rest[..open]);
        let Some(close) = rest[open..].find('>') else {
            rest = "";
            break;
        };
        let tag = &rest[open + 1..open + close];
        if is_spacing_tag(tag) {
            text.push(' ');
        }
        rest = &rest[open + close + 1..];
    }
    text.push_str(rest);
    collapse_whitespace(&decode_entities(&text))
}

/// Whether a tag separates words on screen even though it carries no text of its own.
fn is_spacing_tag(tag: &str) -> bool {
    let name = tag
        .trim_start_matches('/')
        .split(|c: char| c.is_whitespace() || c == '/')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    matches!(name.as_str(), "br" | "p" | "li" | "div" | "tr" | "td" | "th")
}

/// The text strictly between the first `start` and the next `end` after it.
pub fn between<'a>(haystack: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let from = haystack.find(start)? + start.len();
    let len = haystack[from..].find(end)?;
    Some(&haystack[from..from + len])
}

/// The opening tag (`<name …>`) of the first `name` element in the fragment.
pub fn find_tag<'a>(html: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("<{name}");
    let mut from = 0;
    while let Some(found) = html[from..].find(&needle) {
        let start = from + found;
        let after = html[start + needle.len()..].chars().next()?;
        if after.is_whitespace() || after == '>' || after == '/' {
            let end = html[start..].find('>')?;
            return Some(&html[start..=start + end]);
        }
        from = start + needle.len();
    }
    None
}

/// An attribute's decoded value from an opening tag, whichever quote style it uses.
pub fn attribute(tag: &str, name: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let needle = format!(" {name}={quote}");
        if let Some(found) = tag.find(&needle) {
            let from = found + needle.len();
            let len = tag[from..].find(quote)?;
            return Some(decode_entities(&tag[from..from + len]));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_named_and_numeric_entities() {
        assert_eq!(decode_entities("Tom &amp; Jerry &#8211; it&#8217;s &#x2014; fine"), "Tom & Jerry – it’s — fine");
        assert_eq!(decode_entities("a &unknown; b & c"), "a &unknown; b & c");
        assert_eq!(decode_entities("trailing &"), "trailing &");
    }

    #[test]
    fn strips_tags_and_keeps_words_apart() {
        assert_eq!(
            strip_tags("<strong>Explanation:</strong> One<br><br>Two <a href=\"x\">link</a>.\n\t End"),
            "Explanation: One Two link. End"
        );
    }

    #[test]
    fn finds_tags_and_attributes() {
        let html = "<div><iframe loading=\"lazy\" src=\"https://a/b?x=1&amp;y=2\" width='500'></iframe></div>";
        let tag = find_tag(html, "iframe").unwrap();
        assert_eq!(attribute(tag, "src").unwrap(), "https://a/b?x=1&y=2");
        assert_eq!(attribute(tag, "width").unwrap(), "500");
        assert!(attribute(tag, "height").is_none());
        assert!(find_tag("<iframes src=\"x\">", "iframe").is_none());
    }

    #[test]
    fn between_returns_the_inner_slice() {
        assert_eq!(between("a[b]c[d]", "[", "]"), Some("b"));
        assert_eq!(between("a[b", "[", "]"), None);
    }
}
