use super::featch::FetchStrategy;
use super::parser::PageParser;
use reqwest::Client;
use std::sync::Arc;
use tokio;

/// A generic asynchronous web scraping scheduler that can concurrently fetch paginated content
/// using user-defined strategies for page fetching and parsing.
///
/// # Parameters
/// - `start_page`: The starting page index (inclusive).
/// - `end_page`: The ending page index (exclusive).
/// - `page_size`: The number of items per page (passed to the fetch strategy).
/// - `client`: Shared HTTP client (`reqwest::Client`) wrapped in `Arc` for thread-safe reuse.
/// - `parser`: Optional page parser that implements `PageParser` trait.
/// - `strategy`: A fetch strategy that implements the `FetchStrategy` trait.
/// - `site_name`: The name of the site being scraped (used for logging and debugging).
///
/// # Returns
/// A vector of tuples `(index, title, link)` representing parsed page items,
/// sorted by index in ascending order.
pub async fn scrape_site_concurrently(
    start_page: usize,
    end_page: usize,
    page_size: usize,
    client: Arc<Client>,
    parser: Option<Arc<dyn PageParser>>,
    strategy: Arc<dyn FetchStrategy>,
    site_name: &str,
) -> Vec<(usize, String, String)> {
    let mut set = tokio::task::JoinSet::new();

    // Spawn one asynchronous task per page.
    for page in start_page..end_page {
        let client = client.clone();
        let parser = parser.clone();
        let strategy = strategy.clone();
        let site = site_name.to_string();

        set.spawn(async move {
            let res = strategy.fetch_page(page, page_size, client, parser).await;
            if let Err(ref err) = res {
                eprintln!("[{site}] Page {page} fetch failed: {err}");
            }
            res
        });
    }

    let mut result = Vec::new();

    // Await and collect results from all spawned tasks.
    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(mut vec)) => result.append(&mut vec),
            Ok(Err(_)) => {
                // Error already logged inside the task.
            }
            Err(join_err) => {
                // Error occurred while joining the task (e.g., panic).
                eprintln!("[{site_name}] Join error: {join_err}");
            }
        }
    }

    // Sort the results by index for consistent output ordering.
    result.sort_by_key(|x| x.0);
    result
}

/// Specific scraper for Douban Top 250 movie list.
/// Uses 20 pages with 25 items per page.
pub async fn scrape_douban_top250(
    client: Arc<Client>,
    parser: Arc<dyn PageParser>,
    strategy: Arc<dyn FetchStrategy>,
) -> Vec<(usize, String, String)> {
    scrape_site_concurrently(0, 10, 25, client, Some(parser), strategy, "Douban").await
}

/// Specific scraper for IMDb Top 1000 list (split into 4 pages).
/// Uses 1 page per request due to IMDb's structure.
pub async fn scrape_imdb_top1000(
    client: Arc<Client>,
    _parser: Option<Arc<dyn PageParser>>, // Placeholder if IMDb parsing is added in the future
    strategy: Arc<dyn FetchStrategy>,
) -> Vec<(usize, String, String)> {
    scrape_site_concurrently(0, 4, 250, client, None, strategy, "IMDb").await
}
