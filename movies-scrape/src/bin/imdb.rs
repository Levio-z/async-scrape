use easy_scrape::client::get_client;
use std::sync::Arc;
use easy_scrape::featch::ImdbFetchStrategy;
use easy_scrape::featch::LoggingFetchStrategy;
use easy_scrape::scraper::scrape_imdb_top1000;

// main is responsible only for workflow orchestration and final output
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch_strategy = Arc::new(LoggingFetchStrategy::new(Arc::new(ImdbFetchStrategy)));
    let result = scrape_imdb_top1000(get_client(), None, fetch_strategy).await;
    for (i, (_, title, link)) in result.iter().enumerate() {
        println!("{:03}: title:{title:80}  link:{link}", { i + 1 });
    }
    Ok(())
}
