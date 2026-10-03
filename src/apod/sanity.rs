use serde_json::Value;

/// Title NASA's broken APOD scraper puts on every day.
const PLACEHOLDER_TITLE: &str = "NASA Science";

/// Opening words of the site description that scraper uses as the explanation.
const PLACEHOLDER_EXPLANATION: &str = "Discover the cosmos!";

/// Whether an APOD item is the site chrome NASA's API returns when its scraper is broken: the
/// agency logo under the title "NASA Science", the same on every date.
pub fn is_placeholder(item: &Value) -> bool {
    let text = |key: &str| item[key].as_str().unwrap_or_default().trim();
    text("title").eq_ignore_ascii_case(PLACEHOLDER_TITLE)
        || text("explanation").starts_with(PLACEHOLDER_EXPLANATION)
        || ["url", "hdurl"].iter().any(|key| {
            let url = text(key).to_ascii_lowercase();
            url.contains("nasa-logo") || url.contains("nasa_logo")
        })
}

/// Whether an item is a real, displayable APOD entry: the fields the apps render are present and
/// it is not the placeholder.
pub fn is_usable(item: &Value) -> bool {
    let present = |key: &str| item[key].as_str().is_some_and(|v| !v.trim().is_empty());
    item.is_object()
        && ["date", "title", "explanation", "url", "media_type"].iter().all(|key| present(key))
        && !is_placeholder(item)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn good() -> Value {
        json!({
            "date": "2026-10-02",
            "title": "The Complete Sharpless Catalog",
            "explanation": "What does it take to image hundreds of nebulas?",
            "url": "https://assets.science.nasa.gov/dynamicimage/a.png",
            "media_type": "image"
        })
    }

    #[test]
    fn recognises_the_broken_upstream_answer() {
        let broken = json!({
            "date": "2023-01-01",
            "title": "NASA Science",
            "explanation": "Discover the cosmos! Each day a different image",
            "url": "https://science.nasa.gov/wp-content/themes/nasa-child/assets/images/nasa-logo@2x.png",
            "hdurl": "https://science.nasa.gov/wp-content/themes/nasa-child/assets/images/nasa-logo@2x.png",
            "media_type": "image"
        });
        assert!(is_placeholder(&broken));
        assert!(!is_usable(&broken));
    }

    #[test]
    fn any_one_tell_is_enough() {
        let mut item = good();
        item["title"] = json!("nasa science");
        assert!(is_placeholder(&item));

        let mut item = good();
        item["url"] = json!("https://science.nasa.gov/uploads/NASA_logo-1.png");
        assert!(is_placeholder(&item));

        let mut item = good();
        item["explanation"] = json!("Discover the cosmos! Each day");
        assert!(is_placeholder(&item));
    }

    #[test]
    fn accepts_a_real_entry_and_rejects_incomplete_ones() {
        assert!(is_usable(&good()));
        let mut item = good();
        item["url"] = json!("");
        assert!(!is_usable(&item));
        assert!(!is_usable(&json!("string")));
        assert!(!is_usable(&json!({})));
    }
}
