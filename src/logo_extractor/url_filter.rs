/// This module is responsible with ranking every candidate logo URL
/// and picking one that is most likely the correct logo.
/// How should this feature be implemented is explained in notes.md
/// file.

use log::info;
use std::error::Error;

pub fn select_logo_url(urls: &[String]) -> Option<String> {
    info!("Selecting one logo URL from the list");
    // This feature is not implemented.
    // Details are in the documentation.

    // .first() will return a None if the list is empty.
    urls.first().cloned()
}
