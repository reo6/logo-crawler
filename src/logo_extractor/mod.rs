/// This is the root of logo_extractor.

pub mod fetch_html;
pub mod url_filter;

use crate::logo_extractor::fetch_html::fetch_potential_logo_urls;
use crate::logo_extractor::url_filter::select_logo_url;
use log::{info, error, warn, debug};
use std::error::Error;


fn extract_logo_url(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    /// Same as the multiple extractor below, but it takes a single URL.
    /// `extract_logos_from_urls` utilizes this to do its work.

    info!("Extracting logo URL from: {}", url);

    // Fetch all potential candidates for logos and the favicon
    let logo_list_response = fetch_potential_logo_urls(url)?;

    // Select the appropriate logo
    let logo_url = select_logo_url(&logo_list_response.logos);

    // let final_url = logo_url.or(logo_list_response.favicon);
    // This code can be used to return to favicon when a logo was not found.

    Ok(logo_url)
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

            // In case it succeeded the request but failed to find a logo
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

