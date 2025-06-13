use super::utils::parse_titles_from_next_data;
use super::utils::time_it_async;
#[cfg(feature = "output_html")]
use super::utils::write::write_if_not_exists;
use crate::parser::PageParser;
use async_trait::async_trait;
use rand::{Rng, rng};
use reqwest::Client;
use scraper::Html;
use std::sync::Arc;
use tokio::time::Duration;

/// Type alias for the result of a page fetch operation.
/// Encapsulates a vector of tuples (index, title, link) or an error.
/// — Result pattern for error handling and encapsulation.
pub type FetchResult =
    Result<Vec<(usize, String, String)>, Box<dyn std::error::Error + Send + Sync>>;

/// Strategy Pattern Trait: defines the contract for page fetching behavior.
/// Allows multiple interchangeable fetch strategies implementing this trait.
#[async_trait]
pub trait FetchStrategy: Send + Sync {
    async fn fetch_page(
        &self,
        page: usize,
        page_size: usize,
        client: Arc<Client>,
        parser: Option<Arc<dyn PageParser>>,
    ) -> FetchResult;
}

/// Decorator Pattern: Logging enhancement for any FetchStrategy.
/// Adds timing/logging behavior transparently without modifying the core strategy.
pub struct LoggingFetchStrategy {
    inner: Arc<dyn FetchStrategy>,
}

impl LoggingFetchStrategy {
    pub fn new(inner: Arc<dyn FetchStrategy>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl FetchStrategy for LoggingFetchStrategy {
    async fn fetch_page(
        &self,
        page: usize,
        page_size: usize,
        client: Arc<Client>,
        parser: Option<Arc<dyn PageParser>>,
    ) -> FetchResult {
        // Time the fetch_page call and log duration
        time_it_async(format!("Page {page} fetched").as_str(), || async {
            self.inner.fetch_page(page, page_size, client, parser).await
        })
        .await
    }
}

/// Concrete Strategy: Default fetch strategy used for Douban pages.
/// Implements a simple fetch with delay, page calculation, and HTML parsing.
pub struct DefaultFetchStrategy;

#[async_trait]
impl FetchStrategy for DefaultFetchStrategy {
    async fn fetch_page(
        &self,
        page: usize,
        page_size: usize,
        client: Arc<Client>,
        parser: Option<Arc<dyn PageParser>>,
    ) -> FetchResult {
        // Random delay to mimic human-like requests
        let delay = rng().random_range(2..=4);
        tokio::time::sleep(Duration::from_secs(delay)).await;

        // Fetch the page asynchronously with timing
        let resp = time_it_async(format!("Page {page} fetched").as_str(), || async {
            let start = page * page_size;
            let url = format!("https://movie.douban.com/top250?start={start}");
            client.get(&url).send().await
        })
        .await?;

        // Parse the response body using the provided parser
        let res = time_it_async(format!("Page {page} fetched").as_str(), || async {
            let body = resp.text().await?;
            let document = Html::parse_document(body.as_str());
            let parser = parser.ok_or("Parser is required for Douban")?;
            Ok::<Vec<(usize, String, String)>, Box<dyn std::error::Error + Send + Sync>>(
                parser.parse(page, &document),
            )
        })
        .await?;

        Ok(res)
    }
}

/// Concrete Strategy: IMDb fetch strategy with debug output and HTML dumping.
/// Specialized for IMDb page structure and extra debugging.
pub struct ImdbFetchStrategy;

#[async_trait]
impl FetchStrategy for ImdbFetchStrategy {
    async fn fetch_page(
        &self,
        page: usize,
        page_size: usize,
        client: Arc<Client>,
        _parser: Option<Arc<dyn PageParser>>,
    ) -> FetchResult {
        let delay = rng().random_range(2..=4);
        tokio::time::sleep(Duration::from_secs(delay)).await;

        let url = format!(
            "https://www.imdb.com/list/ls048276758/?view=compact&page={}",
            page + 1
        );

        // Fetch IMDb page content with timing
        let res = time_it_async(format!("MDb {page} request").as_str(), || async {
            let resp = client.get(&url).send().await?;
            resp.text().await
        })
        .await?;
        #[cfg(feature = "output_html")]
        {
            // Dump HTML for debugging
            time_it_async(format!("MDb {page} request").as_str(), || async {
                write_if_not_exists("dump_page.html", res.clone().as_str()).await
            })
            .await?;
        }

        // Parse IMDb titles from the fetched HTML content
        time_it_async(format!("Parse {page}").as_str(), || async {
            Ok(parse_titles_from_next_data(page * page_size, &res))
        })
        .await
    }
}
