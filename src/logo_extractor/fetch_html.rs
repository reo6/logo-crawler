/// This module is responsible with the lower level tasks, like making the web
/// request. It also filters the image URLs. This is basically the heart of
/// the whole system.

use ureq;
use scraper::{Html, Selector};
use log::{info, error};
use std::error::Error;
use std::time::Duration;

// Keywords to identify potential logo images
const LOGO_HINTS: [&str; 2] = ["logo", "brand"];
const REQUEST_TIMEOUT: u64 = 4;

fn ensure_https(url: &str) -> String {
    /// Ensuring that the url is valid.

    if url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    }
}

fn extract_favicon_url(document: &Html) -> Option<String> {
    /// This function attempts to extract the favicon URL from the HTML document.
    let link_selector = Selector::parse("link[rel~='icon'], link[rel~='shortcut icon']").unwrap();
    document
        .select(&link_selector)
        .filter_map(|element| element.value().attr("href").map(String::from))
        .next()
}

pub struct LogoListResponse {
    pub logos: Vec<String>,
    pub favicon: Option<String>,
}

pub fn fetch_potential_logo_urls(url: &str) -> Result<LogoListResponse, Box<dyn Error>> {
    /// This makes a web request to the given URL, then
    /// returns every image URL that could be the logo URL.
    /// It filters logos by hints defined in LOGO_HINTS,
    /// checking alt tags, class tags, and the URL itself.

    let url_with_https = ensure_https(url);
    info!("Fetching HTML content from: {}", url_with_https);

    let response = ureq::get(&url_with_https)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .call()
        .map_err(|e| {
            error!("Failed to fetch URL {}: {}", url_with_https, e);
            e
        })?;

    let body = response.into_string().map_err(|e| {
        error!("Failed to read response body for URL {}: {}", url_with_https, e);
        e
    })?;

    let document = Html::parse_document(&body);
    let img_selector = Selector::parse("img").unwrap();

    let urls = document
        .select(&img_selector)
        .filter_map(|element| {
            let element_ref = element.value();
            let class_attr = element_ref.attr("class").unwrap_or_default().to_lowercase();
            let alt_attr = element_ref.attr("alt").unwrap_or_default().to_lowercase();
            let src_attr = element_ref.attr("src").unwrap_or_default().to_lowercase();

            // Check for hints in class, alt, or src attributes
            if LOGO_HINTS.iter().any(|&hint| {
                class_attr.contains(hint) || alt_attr.contains(hint) || src_attr.contains(hint)
            }) {
                element_ref.attr("src").map(String::from)
            } else {
                None
            }
        })
        .collect();

    let favicon = extract_favicon_url(&document);

    Ok(LogoListResponse {
        logos: urls,
        favicon,
    })
}
