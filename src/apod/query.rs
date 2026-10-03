use chrono::{Duration, NaiveDate};

/// WordPress category id of the APOD articles on science.nasa.gov.
const APOD_CATEGORY: u32 = 22766;

/// Posts per WordPress page, the most the REST API allows.
pub const PAGE_SIZE: usize = 100;

/// Longest range one request may ask for, a leap year of pictures.
pub const MAX_RANGE_DAYS: i64 = 366;

/// Pages needed to hold the longest range plus the padding days around it.
pub const MAX_PAGES: u32 = 4;

/// Pages needed to read every post of `start..=end`, at most one post per day.
pub fn pages_for(start: NaiveDate, end: NaiveDate) -> u32 {
    let posts = ((end - start).num_days() + 3).max(1) as usize;
    posts.div_ceil(PAGE_SIZE).clamp(1, MAX_PAGES as usize) as u32
}

/// Most pictures a `count` request may ask for, each costing one upstream fetch.
pub const MAX_COUNT: u32 = 20;

/// Fields requested from WordPress; everything else in a post is page furniture.
const FIELDS: &str = "slug,date,title,content";

/// What a request asks for, in the vocabulary of the classic APOD API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    /// The newest published picture.
    Latest,
    /// One specific day.
    Day(NaiveDate),
    /// Every picture from `start` to `end`, both inclusive.
    Range { start: NaiveDate, end: NaiveDate },
    /// That many pictures from random days of the archive.
    Random(u32),
}

/// The first day APOD was published.
pub fn epoch() -> NaiveDate {
    NaiveDate::from_ymd_opt(1995, 6, 16).expect("valid epoch")
}

impl Query {
    /// Reads the classic parameters (`date`, `start_date`, `end_date`, `count`), rejecting the
    /// combinations the API always rejected. A range is clamped to the archive: it may start before
    /// the first picture and end after today without error.
    pub fn parse(params: &[(String, String)], today: NaiveDate) -> Result<Query, String> {
        let get = |name: &str| params.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str());
        let day = |name: &str| -> Result<Option<NaiveDate>, String> {
            get(name)
                .map(|raw| {
                    NaiveDate::parse_from_str(raw, "%Y-%m-%d")
                        .map_err(|_| format!("{name} must be formatted YYYY-MM-DD"))
                })
                .transpose()
        };
        let (date, start, end) = (day("date")?, day("start_date")?, day("end_date")?);
        let latest_day = today + Duration::days(1);

        if let Some(raw) = get("count") {
            if date.is_some() || start.is_some() || end.is_some() {
                return Err("count cannot be combined with date, start_date or end_date".into());
            }
            let count: u32 = raw.parse().map_err(|_| "count must be a whole number".to_string())?;
            if count == 0 || count > MAX_COUNT {
                return Err(format!("count must be between 1 and {MAX_COUNT}"));
            }
            return Ok(Query::Random(count));
        }
        if let Some(date) = date {
            if start.is_some() || end.is_some() {
                return Err("date cannot be combined with start_date or end_date".into());
            }
            if date < epoch() || date > latest_day {
                return Err(format!("date must be between {} and {}", epoch(), today));
            }
            return Ok(Query::Day(date));
        }
        match (start, end) {
            (None, None) => Ok(Query::Latest),
            (None, Some(_)) => Err("end_date requires start_date".into()),
            (Some(start), end) => {
                let end = end.unwrap_or(today);
                if start > end {
                    return Err("start_date must not be after end_date".into());
                }
                let (start, end) = (start.max(epoch()), end.min(latest_day));
                if (end - start).num_days() >= MAX_RANGE_DAYS {
                    return Err(format!("a range may span at most {MAX_RANGE_DAYS} days"));
                }
                Ok(Query::Range { start, end })
            }
        }
    }

    /// Whether the answer is one object rather than a list.
    pub fn is_single(&self) -> bool {
        matches!(self, Query::Latest | Query::Day(_))
    }

    /// Minutes an answer stays fresh. Recent days can still be corrected or published, so they are
    /// re-read often; settled history is kept for a day. Random picks are never cached.
    pub fn ttl_minutes(&self, today: NaiveDate) -> i64 {
        let newest = match self {
            Query::Latest => return RECENT_TTL_MINUTES,
            Query::Day(day) => *day,
            Query::Range { end, .. } => *end,
            Query::Random(_) => return 0,
        };
        if newest >= today - Duration::days(1) {
            RECENT_TTL_MINUTES
        } else {
            SETTLED_TTL_MINUTES
        }
    }
}

const RECENT_TTL_MINUTES: i64 = 15;
const SETTLED_TTL_MINUTES: i64 = 1440;

/// URL of one page of posts covering `start..=end`. The window is padded by a day each side
/// because WordPress compares against the publication timestamp, and callers keep only posts whose
/// own date falls inside the range.
pub fn range_url(start: NaiveDate, end: NaiveDate, page: u32) -> String {
    let after = format!("{}T12:00:00", start - Duration::days(1));
    let before = format!("{}T12:00:00", end + Duration::days(1));
    format!(
        "https://science.nasa.gov/wp-json/wp/v2/image-article?categories={APOD_CATEGORY}\
         &per_page={PAGE_SIZE}&orderby=date&order=desc&_fields={FIELDS}&after={after}&before={before}&page={page}"
    )
}

/// URL of the newest few published posts. More than one, so a newest post that is missing its
/// picture cannot hide the one before it.
pub fn latest_url() -> String {
    format!(
        "https://science.nasa.gov/wp-json/wp/v2/image-article?categories={APOD_CATEGORY}\
         &per_page={LATEST_POSTS}&orderby=date&order=desc&_fields={FIELDS}"
    )
}

/// Posts read when asking for the latest picture.
const LATEST_POSTS: usize = 5;

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    fn params(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    const TODAY: &str = "2026-10-03";

    fn parse(pairs: &[(&str, &str)]) -> Result<Query, String> {
        Query::parse(&params(pairs), d(TODAY))
    }

    #[test]
    fn no_parameters_means_the_latest_picture() {
        assert_eq!(parse(&[("thumbs", "true"), ("hd", "true")]), Ok(Query::Latest));
    }

    #[test]
    fn a_single_day_must_exist_in_the_archive() {
        assert_eq!(parse(&[("date", "2026-10-02")]), Ok(Query::Day(d("2026-10-02"))));
        assert_eq!(parse(&[("date", "2026-10-04")]), Ok(Query::Day(d("2026-10-04"))));
        assert!(parse(&[("date", "1995-06-15")]).is_err());
        assert!(parse(&[("date", "2026-10-05")]).is_err());
        assert!(parse(&[("date", "10/02/2026")]).is_err());
    }

    #[test]
    fn a_range_defaults_its_end_and_clamps_to_the_archive() {
        assert_eq!(
            parse(&[("start_date", "2026-09-20")]),
            Ok(Query::Range { start: d("2026-09-20"), end: d(TODAY) })
        );
        assert_eq!(
            parse(&[("start_date", "1995-01-01"), ("end_date", "1995-06-20")]),
            Ok(Query::Range { start: epoch(), end: d("1995-06-20") })
        );
        assert_eq!(
            parse(&[("start_date", "2026-09-01"), ("end_date", "2030-01-01")]),
            Ok(Query::Range { start: d("2026-09-01"), end: d("2026-10-04") })
        );
    }

    #[test]
    fn contradictory_parameters_are_rejected() {
        assert!(parse(&[("end_date", "2026-09-20")]).is_err());
        assert!(parse(&[("start_date", "2026-09-20"), ("end_date", "2026-09-01")]).is_err());
        assert!(parse(&[("date", "2026-09-20"), ("start_date", "2026-09-01")]).is_err());
        assert!(parse(&[("count", "3"), ("date", "2026-09-20")]).is_err());
    }

    #[test]
    fn a_range_is_bounded_to_a_year() {
        assert!(parse(&[("start_date", "2025-01-01"), ("end_date", "2025-12-31")]).is_ok());
        assert!(parse(&[("start_date", "2024-01-01"), ("end_date", "2026-01-01")]).is_err());
    }

    #[test]
    fn pages_cover_every_post_of_a_range() {
        assert_eq!(pages_for(d("2026-09-20"), d("2026-10-03")), 1);
        assert_eq!(pages_for(d("2026-10-03"), d("2026-10-03")), 1);
        assert_eq!(pages_for(d("2026-01-01"), d("2026-04-07")), 1);
        assert_eq!(pages_for(d("2026-01-01"), d("2026-04-10")), 2);
        assert_eq!(pages_for(d("2025-01-01"), d("2025-12-31")), 4);
    }

    #[test]
    fn count_is_bounded() {
        assert_eq!(parse(&[("count", "5")]), Ok(Query::Random(5)));
        assert!(parse(&[("count", "0")]).is_err());
        assert!(parse(&[("count", "21")]).is_err());
        assert!(parse(&[("count", "many")]).is_err());
    }

    #[test]
    fn recent_answers_expire_sooner_than_history() {
        let today = d(TODAY);
        assert_eq!(Query::Latest.ttl_minutes(today), 15);
        assert_eq!(Query::Day(d("2026-10-02")).ttl_minutes(today), 15);
        assert_eq!(Query::Day(d("2026-09-01")).ttl_minutes(today), 1440);
        assert_eq!(Query::Range { start: d("2026-09-20"), end: today }.ttl_minutes(today), 15);
        assert_eq!(Query::Range { start: d("2026-08-01"), end: d("2026-08-14") }.ttl_minutes(today), 1440);
        assert_eq!(Query::Random(3).ttl_minutes(today), 0);
    }

    #[test]
    fn range_urls_pad_the_window_by_a_day() {
        let url = range_url(d("2026-09-20"), d("2026-10-03"), 2);
        assert!(url.contains("after=2026-09-19T12:00:00"));
        assert!(url.contains("before=2026-10-04T12:00:00"));
        assert!(url.contains("page=2"));
        assert!(url.contains("categories=22766"));
    }
}
