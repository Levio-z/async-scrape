use scraper::{Html, Selector};
use std::sync::Arc;

/// The Parser interface defines a contract for parsing HTML pages.
/// This enables multiple implementations for different page structures,
/// following the **Strategy Pattern** for interchangeable parsing strategies.
pub trait PageParser: Send + Sync {
    /// Parse the given HTML document for a specific page number.
    /// Returns a vector of tuples: (page_index, title, link).
    fn parse(&self, page: usize, document: &Html) -> Vec<(usize, String, String)>;
}

/// Concrete parser implementation for Douban movie pages.
/// Encapsulates CSS selectors for the main container, title, and link elements.
/// — This is a **Concrete Strategy** implementing the parsing algorithm specific to Douban.
pub struct DoubanParser {
    pub selector: Arc<Selector>,
    pub title_selector: Arc<Selector>,
    pub link_selector: Arc<Selector>,
}

impl PageParser for DoubanParser {
    fn parse(&self, page: usize, document: &Html) -> Vec<(usize, String, String)> {
        let mut results = Vec::new();
        for item in document.select(&self.selector) {
            // Extract the title text using the nested selector.
            let title = item
                .select(&self.title_selector)
                .next()
                .map(|e| e.inner_html())
                .unwrap_or_default();

            // Extract the href attribute from the link element.
            let link = item
                .select(&self.link_selector)
                .next()
                .and_then(|e| e.value().attr("href"))
                .unwrap_or("N/A");

            results.push((page, title, link.into()));
        }
        results
    }
}

/// Factory function to create a shared instance of DoubanParser.
/// Uses Arc smart pointers to enable safe sharing between async tasks or threads.
/// — This follows the **Factory Pattern** by encapsulating construction logic,
/// making client code simpler and decoupled from implementation details.
pub fn create_douban_parser() -> Arc<DoubanParser> {
    Arc::new(DoubanParser {
        selector: Arc::new(Selector::parse("div.item").unwrap()),
        title_selector: Arc::new(Selector::parse("div.hd > a > span.title").unwrap()),
        link_selector: Arc::new(Selector::parse("div.hd > a").unwrap()),
    })
}
