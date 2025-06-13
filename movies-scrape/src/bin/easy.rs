// id=1 len=381
// id=3 len=381
// id=2 len=381
// id=0 len=381
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    // Create a factory for HTTP clients (encapsulates connection details)
    let client_factory = Arc::new(HttpClientFactory::new("110.242.68.66:80", "baidu.com"));

    let mut set = JoinSet::new();

    // Spawn multiple concurrent HTTP GET tasks
    for id in 0..4 {
        let factory = Arc::clone(&client_factory);
        set.spawn(async move { HttpScraper::new(id, factory).scrape().await });
    }

    // Process task results as they complete
    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(result)) => println!("id={} len={}", result.id, result.len),
            Ok(Err(e)) => eprintln!("Task error: {e}"),
            Err(join_err) => eprintln!("Join error: {join_err}"),
        }
    }
}

/// Factory to create TCP connections configured for the target HTTP server
struct HttpClientFactory {
    addr: String,
    host_header: String,
}

impl HttpClientFactory {
    fn new(addr: &str, host_header: &str) -> Self {
        Self {
            addr: addr.to_string(),
            host_header: host_header.to_string(),
        }
    }

    /// Establishes a TCP connection asynchronously
    async fn connect(
        &self,
    ) -> Result<tokio::net::TcpStream, Box<dyn std::error::Error + Send + Sync>> {
        let socket = tokio::net::TcpSocket::new_v4()?;
        let stream = socket.connect(self.addr.parse()?).await?;
        Ok(stream)
    }

    fn host_header(&self) -> &str {
        &self.host_header
    }
}

/// Represents a single scraping task
struct HttpScraper {
    id: usize,
    client_factory: Arc<HttpClientFactory>,
}

impl HttpScraper {
    fn new(id: usize, client_factory: Arc<HttpClientFactory>) -> Self {
        Self { id, client_factory }
    }
    /// Executes the HTTP GET request and returns the result
    async fn scrape(&self) -> Result<ScrapeResult, Box<dyn std::error::Error + Send + Sync>> {
        let mut stream = self.client_factory.connect().await?;

        let request = format!(
            "GET / HTTP/1.1\r\nHost: {}\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\nConnection: close\r\n\r\n",
            self.client_factory.host_header()
        );

        stream.write_all(request.as_bytes()).await?;

        let mut response = String::new();
        stream.read_to_string(&mut response).await?;

        Ok(ScrapeResult {
            id: self.id,
            len: response.len(),
        })
    }
}

/// Holds the scraping result data
struct ScrapeResult {
    id: usize,
    len: usize,
}
