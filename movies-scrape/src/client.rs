use reqwest::{Client, header};
use std::sync::{Arc, LazyLock};

/// Factory method to build a common set of HTTP headers.
/// Encapsulates the creation of standard headers, so callers don't deal with details.
/// — Factory Pattern
fn common_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"),
    );
    headers
}

/// Global lazily initialized singleton HTTP client instance, thread-safe.
/// — Singleton Pattern
/// Ensures only one Client instance exists throughout the program lifetime,
/// avoiding repeated costly creation and unnecessary connection pools.
static HTTP_CLIENT: LazyLock<Arc<Client>> = LazyLock::new(|| {
    let client = Client::builder()
        .default_headers(common_headers()) // Inject unified header config
        .build()
        .expect("Failed to build HTTP client");
    Arc::new(client) // Wrap in Arc for thread-safe sharing
});

/// Accessor method for the global HTTP client singleton.
/// — Accessor Pattern (a form of controlled access)
/// Returns a cloned Arc<Client>, allowing safe shared ownership,
/// and hides singleton implementation details.
pub fn get_client() -> Arc<Client> {
    HTTP_CLIENT.clone()
}
