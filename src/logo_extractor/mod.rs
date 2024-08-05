/// This is the root of logo_extractor.

pub mod fetch_html;
pub mod url_filter;

use crate::logo_extractor::fetch_html::fetch_potential_logo_urls;
use crate::logo_extractor::url_filter::select_logo_url;
use log::{info, error, warn, debug};
use std::error::Error;


fn ensure_https_scheme(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    }
}

fn resolve_relative_url(base_url: &str, relative_url: &str) -> String {
    if relative_url.starts_with("http://") || relative_url.starts_with("https://") {
        // If the relative_url is already an absolute URL, return it as-is
        return relative_url.to_string();
    }

    let base_url = ensure_https_scheme(base_url);
    let base_url_trimmed = base_url.trim_end_matches('/');
    let relative_url_trimmed = relative_url.trim_start_matches('/');

    // Combine base_url and relative_url, ensuring no double slashes
    format!("{}/{}", base_url_trimmed, relative_url_trimmed)
}

fn extract_logo_url(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    /// Same as the multiple extractor below, but it takes a single URL.
    /// `extract_logos_from_urls` utilizes this to do its work.

    info!("Extracting logo URL from: {}", url);

    // Fetch all potential candidates for logos and the favicon
    let logo_list_response = fetch_potential_logo_urls(url)?;

    // Select the appropriate logo URL or favicon
    let mut logo_url = select_logo_url(&logo_list_response.logos);
    if logo_url.is_none() {
        logo_url = logo_list_response.favicon;
    }

    // Resolve the URL if it is relative
    if let Some(logo_url) = logo_url {
        let resolved_url = resolve_relative_url(url, &logo_url);
        return Ok(Some(resolved_url));
    }

    Ok(None)
}

pub struct ExtractionResult {
    pub num_errors: usize, // Number of times the extractor failed
    pub num_not_found: usize, // Number of times the program failed to find a logo
    pub num_successful: usize, // Number of times the program successfully found one or more logos
    pub logo_urls: Vec<String>,
}

pub fn extract_logos_from_urls(urls: &[&str]) -> Result<ExtractionResult, Box<dyn Error>> {
    /// This takes multiple URLs and uses fetch_html to
    /// extract logo URL. This is the final point
    /// before main.


    let mut result = ExtractionResult {
        num_errors: 0,
        num_not_found: 0,
        num_successful: 0,
        logo_urls: Vec::new(),
    };

    debug!("Beginning the extraction of multiple URLs.");

    for &url in urls {
        info!("Processing URL: {}", url);
        match extract_logo_url(url) {
            // In case it succeeded and there is a logo
            Ok(Some(logo_url)) => {
                result.logo_urls.push(logo_url);
                result.num_successful += 1;
            }

            // In case the request was successful but the algorithm failed to find a logo
            Ok(None) => {
                result.num_not_found += 1;
                warn!("No logo found for URL: {}", url);
            }

            // In case there was an error
            Err(e) => {
                result.num_errors += 1;
                error!("Failed to extract logo from URL {}: {}", url, e);
            }
        }
    }

    Ok(result)
}

