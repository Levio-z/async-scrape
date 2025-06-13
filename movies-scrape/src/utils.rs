use tokio;
use tokio::time::Instant;
use unicode_width::UnicodeWidthStr;

/// Writes content to a file only if the file does NOT already exist.
/// If the file exists, the write operation is skipped to avoid overwriting.
///
/// # Arguments
///
/// * `path` - The file path to check and potentially write to.
/// * `content` - The string content to write into the file.
///
/// # Returns
///
/// * `std::io::Result<()>` - Ok if successful, or an IO error otherwise.
#[cfg(feature = "output_html")]
pub mod write {
    use std::path::Path;
    use tokio::fs;
    use tokio::io::AsyncWriteExt;
    pub async fn write_if_not_exists(
        path: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("File '{path}' strat.");
        if !Path::new(path).exists() {
            // File does not exist: create and write content.
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(path)
                .await?;
            file.write_all(content.as_bytes()).await?;
            println!("File '{path}' did not exist. Content written.");
        } else {
            // File exists: skip writing to prevent overwriting.
            println!("File '{path}' already exists. Write operation skipped.");
        }
        Ok(())
    }
}

/// Parses a simplified `__NEXT_DATA__` JSON-like string from IMDb's HTML response
/// to extract (index, title, url) tuples.
///
/// # Arguments
/// * `json_str` - Raw JSON string embedded in a <script> tag
///
/// # Returns
/// A list of (index, title, url) tuples
pub fn parse_titles_from_next_data(
    start_index: usize,
    json_str: &str,
) -> Vec<(usize, String, String)> {
    let mut res = Vec::new();
    let mut pos = 0;
    let mut index = 1;

    while let Some(start) = json_str[pos..].find("@type\":\"ListItem\",\"item\":{\"@type\":\"") {
        // Move position after the matched pattern
        let slice = &json_str[pos + start + 35..];

        // Extract URL
        if let Some(id_pos) = slice.find("url\":\"") {
            let url_start = id_pos + 6;
            if let Some(url_end_offset) = slice[url_start..].find('"') {
                let url = &slice[url_start..url_start + url_end_offset];

                // Try to find title after the URL
                let title_key = "\"name\":\"";
                if let Some(title_pos) = slice[url_start + url_end_offset..].find(title_key) {
                    let title_start = url_start + url_end_offset + title_pos + title_key.len();
                    if let Some(title_end_offset) = slice[title_start..].find('"') {
                        let title = &slice[title_start..title_start + title_end_offset];

                        res.push((start_index + index, title.to_string(), url.to_string()));
                        index += 1;
                    }
                }
            }
        }

        // Move the search cursor forward to avoid infinite loop
        pos += start + 50;
    }

    res
}

pub async fn time_it_async<T, Fut, F>(label: &str, f: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let start = Instant::now();
    let result = f().await;
    println!("{} took: {:.2?}", label, start.elapsed());
    result
}

/// Prints the movie info with aligned title column accounting for wide characters (e.g., Chinese).
///
/// # Parameters
/// - `i`: The index or rank number (will be zero-padded to 3 digits).
/// - `title`: The movie title string, may contain wide characters.
/// - `link`: The URL link associated with the movie.
///
/// # Details
/// This function uses the `unicode-width` crate to calculate the displayed width of `title`,
/// which correctly counts wide characters like Chinese as width 2 instead of 1.
/// It then pads the title with spaces so that the next column (`link`) aligns properly
/// in a monospaced terminal output.
///
/// Without this approach, formatting with `{:<width}` may misalign columns due to multi-byte characters.
pub fn print_aligned(i: usize, title: &str, link: &str) {
    let width = 30; // 期望宽度
    let title_width = UnicodeWidthStr::width(title);
    let padding = (width as usize).saturating_sub(title_width);
    let padding_spaces = " ".repeat(padding);
    println!("{i:03}: {title}{padding_spaces} {link}");
}
