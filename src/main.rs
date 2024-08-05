use env_logger;
use log::{info, error};
use std::error::Error;
use std::io::{self, BufRead};
use csv::Writer;

mod logo_extractor;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Read URLs from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();
    let urls: Vec<String> = handle
        .lines()
        .filter_map(Result::ok)
        .collect();

    if urls.is_empty() {
        error!("No URLs provided. Please provide URLs via stdin.");

        // This is like the "return 0" in C.
        // But it uses Result of Rust, and it looks much cleaner!
        // How cool is that?
        return Ok(());
    }

    // Extract logos from URLs
    let result = logo_extractor::extract_logos_from_urls(&urls.iter().map(AsRef::as_ref).collect::<Vec<&str>>())?;

    // Output the result to stdout as CSV
    let mut wtr = Writer::from_writer(io::stdout());

    wtr.write_record(&["Logo URL"])?;
    for url in result.logo_urls {
        wtr.write_record(&[url])?;
    }

    wtr.flush()?;
    info!("CSV output written successfully.");

    info!("Successful: {}", result.num_successful);
    info!("Error: {}", result.num_errors);
    info!("Failed: {}", result.num_not_found);

    Ok(())
}
