use easy_scrape::client::get_client;
use easy_scrape::featch::DefaultFetchStrategy;
use easy_scrape::parser::create_douban_parser;
use easy_scrape::scraper::scrape_douban_top250;
use easy_scrape::utils::print_aligned;
use std::sync::Arc;

// main is only responsible for orchestrating the workflow and producing the final output
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch_strategy = Arc::new(DefaultFetchStrategy);
    let result = scrape_douban_top250(get_client(), create_douban_parser(), fetch_strategy).await;
    for (i, (_, title, link)) in result.into_iter().enumerate() {
        print_aligned(i + 1, &title, &link);
    }
    Ok(())
}
