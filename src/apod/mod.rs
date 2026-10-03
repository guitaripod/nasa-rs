//! Astronomy Picture of the Day, read from NASA's own publication on science.nasa.gov.
//!
//! NASA retired `apod.nasa.gov` in 2026 and the `api.nasa.gov/planetary/apod` service now answers
//! every date with the site chrome ("NASA Science" plus the agency logo) instead of the picture.
//! The pictures live on as WordPress posts, so this module turns those posts back into the classic
//! APOD JSON and recognises the broken upstream answer so it is never shown or cached.

/// Entity decoding, tag stripping and attribute lookup for the few HTML shapes the posts use.
pub mod html;
/// One APOD post turned into the classic APOD JSON.
pub mod entry;
/// Request parameters turned into a plan, plus the science.nasa.gov URLs that serve it.
pub mod query;
/// Recognises the placeholder answer NASA's own API gives when its scraper is broken.
pub mod sanity;
