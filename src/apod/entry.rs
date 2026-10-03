use chrono::NaiveDate;
use serde_json::{json, Map, Value};

use super::html::{attribute, between, collapse_whitespace, decode_entities, find_tag, strip_tags};

/// Widest picture the grid and detail screens need; the full-size file stays available as `hdurl`.
const WEB_WIDTH: u32 = 1280;

/// Hosts whose image URLs accept `?w=` resizing, so large originals can be served at web size.
const RESIZABLE_HOST: &str = "assets.science.nasa.gov";

/// Markers at the end of every post's description that belong to the page, not the explanation.
const FOOTER_MARKERS: [&str; 7] = [
    "<strong>APOD",
    "APOD&#8217;s email",
    "APOD's email",
    "<strong>Tomorrow",
    "Tomorrow&#8217;s picture",
    "Tomorrow's picture",
    "Tomorrow&#8217;s APOD",
];

/// What went wrong turning a post into an entry. Callers skip the post and log the reason.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// The post is not an APOD article (wrong slug) or carries no usable date.
    NotAnApod,
    /// A field every entry needs is missing or empty.
    Missing(&'static str),
    /// The post shows no picture, video or embed.
    NoMedia,
}

/// The picture, video or embed an entry shows.
#[derive(Debug, PartialEq, Eq)]
pub enum Media {
    /// A still image: `url` at web size, `hdurl` the full-size file.
    Image { url: String, hdurl: String },
    /// A playable video, with a poster still when one can be derived.
    Video { url: String, thumbnail: Option<String> },
    /// An interactive embed the apps cannot play inline.
    Other { url: String },
}

/// One day's picture in the shape the classic APOD API served it.
#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub date: NaiveDate,
    pub title: String,
    pub explanation: String,
    pub copyright: Option<String>,
    pub media: Media,
}

impl Entry {
    /// Reads one `image-article` post from the WordPress REST API.
    pub fn from_post(post: &Value) -> Result<Entry, ParseError> {
        let slug = post["slug"].as_str().unwrap_or_default();
        if !slug.starts_with("apod-") {
            return Err(ParseError::NotAnApod);
        }
        let date = post_date(post).ok_or(ParseError::NotAnApod)?;
        let content = post["content"]["rendered"].as_str().unwrap_or_default();

        let title = title(post, content).ok_or(ParseError::Missing("title"))?;
        let explanation = explanation(content).ok_or(ParseError::Missing("explanation"))?;
        let media = media(content).ok_or(ParseError::NoMedia)?;
        let copyright = credit(content).and_then(|text| copyright_from_credit(&text));

        Ok(Entry { date, title, explanation, copyright, media })
    }

    /// The classic APOD JSON object. `thumbs` adds a poster still for videos, as the old API did.
    pub fn to_json(&self, thumbs: bool) -> Value {
        let mut out = Map::new();
        if let Some(copyright) = &self.copyright {
            out.insert("copyright".into(), json!(copyright));
        }
        out.insert("date".into(), json!(self.date.format("%Y-%m-%d").to_string()));
        out.insert("explanation".into(), json!(self.explanation));
        match &self.media {
            Media::Image { url, hdurl } => {
                out.insert("hdurl".into(), json!(hdurl));
                out.insert("media_type".into(), json!("image"));
                out.insert("url".into(), json!(url));
            }
            Media::Video { url, thumbnail } => {
                out.insert("media_type".into(), json!("video"));
                out.insert("url".into(), json!(url));
                if let (true, Some(thumbnail)) = (thumbs, thumbnail) {
                    out.insert("thumbnail_url".into(), json!(thumbnail));
                }
            }
            Media::Other { url } => {
                out.insert("media_type".into(), json!("other"));
                out.insert("url".into(), json!(url));
            }
        }
        out.insert("service_version".into(), json!("v1"));
        out.insert("title".into(), json!(self.title));
        Value::Object(out)
    }
}

/// The APOD day a post belongs to: the calendar date of its publication stamp.
fn post_date(post: &Value) -> Option<NaiveDate> {
    let stamp = post["date"].as_str()?;
    NaiveDate::parse_from_str(stamp.get(..10)?, "%Y-%m-%d").ok()
}

/// The picture's title, from the page heading and falling back to the post title minus its
/// `APOD: <date> – ` prefix.
fn title(post: &Value, content: &str) -> Option<String> {
    let heading = between(content, "<h1", "</h1>")
        .and_then(|inner| inner.split_once('>'))
        .map(|(_, text)| strip_tags(text));
    let from_post = || {
        let rendered = decode_entities(post["title"]["rendered"].as_str()?);
        let title = match rendered.split_once(" – ") {
            Some((_, rest)) => rest.to_string(),
            None => rendered,
        };
        Some(collapse_whitespace(&title))
    };
    heading.filter(|t| !t.is_empty()).or_else(from_post).filter(|t| !t.is_empty())
}

/// The astronomer's explanation as plain text, without the page's submission and next-day notes.
fn explanation(content: &str) -> Option<String> {
    let start = content.find("media-detail-hero__description")?;
    let body = &content[start..];
    let open = body.find('>')? + 1;
    let end = body.find("</p>")?;
    let mut html = &body[open..end.max(open)];
    if let Some(cut) = FOOTER_MARKERS.iter().filter_map(|marker| html.find(marker)).min() {
        html = &html[..cut];
    }
    let text = strip_tags(html);
    let text = text.strip_prefix("Explanation:").unwrap_or(&text).trim().to_string();
    (!text.is_empty()).then_some(text)
}

/// The credit line from the post's metadata table, as plain text.
fn credit(content: &str) -> Option<String> {
    content.split("media-detail-hero__meta-row").skip(1).find_map(|row| {
        let row = row.split("</tr>").next()?;
        let label = strip_tags(between(row, "<th", "</th>")?.split_once('>')?.1);
        if !label.to_lowercase().contains("credit") {
            return None;
        }
        let value = strip_tags(between(row, "<td", "</td>")?.split_once('>')?.1);
        (!value.is_empty()).then_some(tidy_punctuation(&value))
    })
}

/// Removes the stray spaces the editors leave before commas and semicolons.
fn tidy_punctuation(text: &str) -> String {
    text.replace(" ,", ",").replace(" ;", ";").replace(" .", ".")
}

/// The copyright holder, the way the old API reported it: present only for credit lines that
/// declare a copyright or name a bare photographer, absent for plain agency credits.
fn copyright_from_credit(credit: &str) -> Option<String> {
    if let Some((label, rest)) = credit.split_once(':') {
        let lowered = label.to_lowercase();
        if label.len() <= 40 && lowered.contains("credit") {
            let rest = rest.trim();
            return (lowered.contains("copyright") && !rest.is_empty()).then(|| rest.to_string());
        }
    }
    (!credit.is_empty()).then(|| credit.to_string())
}

/// The hero media of a post: an embed, a video file or a picture.
fn media(content: &str) -> Option<Media> {
    let start = content.find("media-detail-hero__media")?;
    let region = &content[start..];
    let end = ["<h1", "media-detail-hero__description"]
        .iter()
        .filter_map(|marker| region.find(marker))
        .min()
        .unwrap_or(region.len());
    let region = &region[..end];

    if let Some(iframe) = find_tag(region, "iframe") {
        return attribute(iframe, "src").filter(|src| !src.is_empty()).map(embed);
    }
    if region.contains("<video") {
        let source = find_tag(region, "source").and_then(|tag| attribute(tag, "src"));
        let video = find_tag(region, "video").and_then(|tag| attribute(tag, "src"));
        let url = source.or(video).filter(|url| !url.is_empty())?;
        return Some(Media::Video { url, thumbnail: None });
    }
    let Some(image) = find_tag(region, "img") else {
        return bare_embed_url(region).map(embed);
    };
    let src = attribute(image, "src").filter(|src| !src.is_empty())?;
    let full = find_tag(region, "a")
        .and_then(|tag| attribute(tag, "href"))
        .filter(|href| href.starts_with("http"))
        .unwrap_or(src);
    let width = attribute(image, "width").and_then(|w| w.parse::<u32>().ok());
    Some(Media::Image { url: web_sized(&full, width), hdurl: full })
}

/// The link WordPress leaves as plain text inside an embed block when it could not build a player.
fn bare_embed_url(region: &str) -> Option<String> {
    let wrapper = between(region, "wp-block-embed__wrapper\">", "</div>")?;
    let text = strip_tags(wrapper);
    text.starts_with("http").then(|| text.split_whitespace().next().unwrap_or_default().to_string())
}

/// Classifies an embed: YouTube and Vimeo play inline, anything else is an interactive embed.
fn embed(src: String) -> Media {
    if let Some(id) = youtube_id(&src) {
        return Media::Video {
            url: format!("https://www.youtube.com/embed/{id}?rel=0"),
            thumbnail: Some(format!("https://img.youtube.com/vi/{id}/hqdefault.jpg")),
        };
    }
    if let Some(id) = vimeo_id(&src) {
        return Media::Video { url: format!("https://player.vimeo.com/video/{id}"), thumbnail: None };
    }
    Media::Other { url: src }
}

/// The characters of an id token at the start of `rest`.
fn id_token(rest: &str) -> String {
    rest.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-').collect()
}

/// The video id of a YouTube embed, watch or short URL.
fn youtube_id(src: &str) -> Option<String> {
    let id = ["youtube.com/embed/", "youtube-nocookie.com/embed/", "youtu.be/"]
        .iter()
        .find_map(|marker| src.split_once(marker).map(|(_, rest)| id_token(rest)))
        .or_else(|| {
            let query = src.split_once("youtube.com/watch")?.1.split_once('?')?.1;
            query.split('&').find_map(|pair| pair.strip_prefix("v=").map(id_token))
        })?;
    (!id.is_empty()).then_some(id)
}

/// The numeric id of a Vimeo player or page URL.
fn vimeo_id(src: &str) -> Option<String> {
    let rest = ["player.vimeo.com/video/", "vimeo.com/"].iter().find_map(|marker| src.split_once(marker).map(|(_, rest)| rest))?;
    let id: String = rest.chars().take_while(char::is_ascii_digit).collect();
    (!id.is_empty()).then_some(id)
}

/// A web-sized rendition of a large NASA original, or the URL unchanged when it is small enough
/// or served from somewhere that cannot resize.
fn web_sized(full: &str, width: Option<u32>) -> String {
    match width {
        Some(w) if w > WEB_WIDTH && full.contains(RESIZABLE_HOST) => {
            let base = full.split('?').next().unwrap_or(full);
            format!("{base}?w={WEB_WIDTH}&fit=clip")
        }
        _ => full.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Value {
        let path = format!("{}/tests/fixtures/apod/{name}.json", env!("CARGO_MANIFEST_DIR"));
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn parses_an_image_day() {
        let entry = Entry::from_post(&fixture("image")).unwrap();
        assert_eq!(entry.date.to_string(), "2026-10-02");
        assert_eq!(entry.title, "The Complete Sharpless Catalog: 313 Nebulas");
        assert!(entry.explanation.starts_with("What does it take to image hundreds of nebulas?"));
        assert_eq!(entry.copyright.as_deref(), Some("Bing Xin"));
        let Media::Image { url, hdurl } = &entry.media else { panic!("expected an image") };
        assert!(url.ends_with("/sharpless_catalog.png?w=1280&fit=clip"), "{url}");
        assert!(hdurl.contains("w=4455"), "{hdurl}");
    }

    #[test]
    fn drops_the_page_footer_from_the_explanation() {
        let entry = Entry::from_post(&fixture("image_credit_nasa")).unwrap();
        assert!(entry.explanation.ends_with("up the slope of Mount Sharp."), "{}", entry.explanation);
        assert!(!entry.explanation.contains("Tomorrow"));
        assert!(!entry.explanation.contains("submissions"));
        assert!(entry.explanation.contains("rover’s Mastcam"));
    }

    #[test]
    fn agency_credits_carry_no_copyright() {
        let entry = Entry::from_post(&fixture("image_credit_nasa")).unwrap();
        assert_eq!(entry.copyright, None);
    }

    #[test]
    fn parses_a_youtube_day_with_a_poster() {
        let entry = Entry::from_post(&fixture("youtube")).unwrap();
        assert_eq!(entry.title, "A Double Sunrise from a Partial Eclipse");
        assert!(entry.copyright.as_deref().unwrap().starts_with("Jason Kurth; Music:"), "{:?}", entry.copyright);
        let Media::Video { url, thumbnail } = &entry.media else { panic!("expected a video") };
        assert_eq!(url, "https://www.youtube.com/embed/oTkbHJsqCZM?rel=0");
        assert_eq!(thumbnail.as_deref(), Some("https://img.youtube.com/vi/oTkbHJsqCZM/hqdefault.jpg"));
    }

    #[test]
    fn parses_a_direct_video_file() {
        let entry = Entry::from_post(&fixture("mp4")).unwrap();
        let Media::Video { url, thumbnail } = &entry.media else { panic!("expected a video") };
        assert!(url.ends_with("LunarEclipseUruguay_Salazar.mp4"), "{url}");
        assert!(thumbnail.is_none());
    }

    #[test]
    fn treats_a_foreign_iframe_as_an_interactive_embed() {
        let entry = Entry::from_post(&fixture("interactive")).unwrap();
        assert!(matches!(entry.media, Media::Other { .. }));
        assert_eq!(entry.to_json(true)["media_type"], "other");
    }

    #[test]
    fn keeps_small_vintage_images_as_published() {
        let entry = Entry::from_post(&fixture("vintage")).unwrap();
        assert_eq!(entry.date.to_string(), "1995-06-16");
        assert_eq!(entry.title, "Neutron Star Earth");
        assert_eq!(entry.copyright, None);
        let Media::Image { url, hdurl } = &entry.media else { panic!("expected an image") };
        assert_eq!(url, hdurl);
        assert!(url.contains("/1995/june/e_lens.gif"));
    }

    #[test]
    fn serialises_the_classic_shape() {
        let json = Entry::from_post(&fixture("image")).unwrap().to_json(true);
        for key in ["copyright", "date", "explanation", "hdurl", "media_type", "service_version", "title", "url"] {
            assert!(json.get(key).is_some(), "missing {key}");
        }
        assert_eq!(json["media_type"], "image");
        assert!(json.get("thumbnail_url").is_none());
    }

    #[test]
    fn only_videos_get_a_thumbnail_and_only_on_request() {
        let entry = Entry::from_post(&fixture("youtube")).unwrap();
        assert!(entry.to_json(true).get("thumbnail_url").is_some());
        assert!(entry.to_json(false).get("thumbnail_url").is_none());
        assert!(entry.to_json(true).get("hdurl").is_none());
    }

    #[test]
    fn rejects_posts_that_are_not_apod_articles() {
        let mut post = fixture("image");
        post["slug"] = json!("some-other-article");
        assert_eq!(Entry::from_post(&post), Err(ParseError::NotAnApod));
    }

    #[test]
    fn rejects_posts_without_media_or_explanation() {
        let mut post = fixture("image");
        post["content"]["rendered"] = json!("<div class=\"media-detail-hero__media\"></div><h1>T</h1>");
        assert_eq!(Entry::from_post(&post), Err(ParseError::Missing("explanation")));
    }

    #[test]
    fn credit_labels_decide_the_copyright() {
        assert_eq!(copyright_from_credit("Image Credit: NASA, JPL-Caltech"), None);
        assert_eq!(copyright_from_credit("Image Credit & Copyright: Jane Doe").as_deref(), Some("Jane Doe"));
        assert_eq!(copyright_from_credit("Jane Doe").as_deref(), Some("Jane Doe"));
        assert_eq!(copyright_from_credit(""), None);
    }

    #[test]
    fn reads_video_links_that_wordpress_left_as_text() {
        let region = "<figure class=\"wp-block-embed\"><div class=\"wp-block-embed__wrapper\">\n https://www.youtube.com/watch?v=M6-iC_aYcug \n</div></figure>";
        let Some(Media::Video { url, thumbnail }) = bare_embed_url(region).map(embed) else { panic!("expected a video") };
        assert_eq!(url, "https://www.youtube.com/embed/M6-iC_aYcug?rel=0");
        assert_eq!(thumbnail.as_deref(), Some("https://img.youtube.com/vi/M6-iC_aYcug/hqdefault.jpg"));

        let region = "<div class=\"wp-block-embed__wrapper\">https://player.vimeo.com/video/23152199?title=0&#038;byline=0</div>";
        assert_eq!(
            bare_embed_url(region).map(embed),
            Some(Media::Video { url: "https://player.vimeo.com/video/23152199".into(), thumbnail: None })
        );
    }

    #[test]
    fn recognises_every_youtube_url_shape() {
        for src in [
            "https://www.youtube.com/embed/abc_123-XY?feature=oembed",
            "https://www.youtube-nocookie.com/embed/abc_123-XY",
            "https://youtu.be/abc_123-XY?t=4",
            "https://www.youtube.com/watch?feature=share&v=abc_123-XY&t=9",
        ] {
            assert_eq!(youtube_id(src).as_deref(), Some("abc_123-XY"), "{src}");
        }
        assert_eq!(youtube_id("https://example.com/watch?v=abc"), None);
    }

    #[test]
    fn empty_media_sources_are_treated_as_no_media() {
        let content = |hero: &str| format!("<div class=\"media-detail-hero__media\">{hero}</div><h1>T</h1>");
        assert_eq!(media(&content("<video><source src=\"\" type=\"video/mp4\"></video>")), None);
        assert_eq!(media(&content("<figure><a href=\"\"></a></figure>")), None);
        assert_eq!(media(&content("<iframe src=\"\"></iframe>")), None);
    }

    #[test]
    fn web_sizing_only_touches_large_resizable_originals() {
        let big = "https://assets.science.nasa.gov/dynamicimage/a/b.jpg?w=4000&h=3000&fit=clip";
        assert_eq!(web_sized(big, Some(4000)), "https://assets.science.nasa.gov/dynamicimage/a/b.jpg?w=1280&fit=clip");
        assert_eq!(web_sized(big, Some(1000)), big);
        assert_eq!(web_sized(big, None), big);
        assert_eq!(web_sized("https://example.com/x.jpg", Some(5000)), "https://example.com/x.jpg");
    }
}

#[cfg(test)]
mod corpus {
    use super::*;
    use crate::apod::sanity;

    #[test]
    #[ignore = "needs APOD_CORPUS=<file holding a JSON array of WordPress posts>"]
    fn parses_a_corpus_of_real_posts() {
        let path = std::env::var("APOD_CORPUS").expect("APOD_CORPUS");
        let posts: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let mut failures = Vec::new();
        let (mut images, mut videos, mut others, mut copyrights) = (0, 0, 0, 0);
        for post in &posts {
            match Entry::from_post(post) {
                Ok(entry) => {
                    let json = entry.to_json(true);
                    if !sanity::is_usable(&json) {
                        failures.push(format!("{} unusable: {json}", post["slug"]));
                    }
                    copyrights += entry.copyright.is_some() as u32;
                    match entry.media {
                        Media::Image { .. } => images += 1,
                        Media::Video { .. } => videos += 1,
                        Media::Other { .. } => others += 1,
                    }
                }
                Err(reason) => failures.push(format!("{} {reason:?}", post["slug"])),
            }
        }
        println!("posts={} images={images} videos={videos} others={others} with_copyright={copyrights}", posts.len());
        for failure in failures.iter().take(25) {
            println!("FAIL {failure}");
        }
        assert!(failures.len() * 100 <= posts.len(), "{} of {} posts failed", failures.len(), posts.len());
    }
}
