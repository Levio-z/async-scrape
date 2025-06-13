# Rust Async Web Crawler: IMDb & Douban

This project is a high-performance, modular, fully asynchronous crawler written in **Rust**, capable of scraping the **IMDb Top 1000** and **Douban Movie Rankings**.

> 🚀 Designed with performance, scalability, and clean architecture in mind.

---

## ✨ Features

- ✅ Asynchronous HTTP requests powered by `reqwest` and `tokio`
- ✅ HTML parsing using CSS-like selectors via `scraper`
- ✅ **Strategy pattern** to configure pluggable fetch behavior
- ✅ `PageParser` trait for modular IMDb / Douban parsing
- ✅ Built-in logging,and HTML dump for debugging
- ✅ Clean architecture with extensibility in mind

---

## 📦 Dependencies

```toml
# Cargo.toml

[dependencies]
reqwest = "0.12.20"
scraper = "0.23.1"
tokio = { version = "1.36", features = ["full"] }
futures = "0.3"
rand = "0.9.1"
async-trait = "0.1"
```

---

## 🚀 Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/Levio-z/async-scrape
cd async-scrape
```

### 2. Run the IMDb crawler

```bash
cargo run --bin imdb
```
### 3. Run the IMDb crawler with HTML output enabled
```bash
cargo run --bin imdb --features output_html

```

### 4. Run the Douban crawler

```bash
cargo run --bin douban
```

> 💡 Recommended layout uses Rust multi-bin structure: `src/bin/imdb.rs`, `src/bin/douban.rs`

---

## 🧠 Design Patterns Used

| Pattern         | Usage Description                                                                 |
|----------------|-------------------------------------------------------------------------------------|
| **Strategy**    | `FetchStrategy` enables flexible control over request behavior (e.g., logging, proxy) |
| **Singleton**   | `LazyLock<Arc<Client>>` provides global HTTP client instance                        |
| **Decorator**   | Wraps fetch strategy for extended behavior (e.g., logging)                          |
| **Factory**     | `create_douban_parser()`, `create_client()` centralize instantiation logic          |
| **Trait Object**| `PageParser` allows runtime selection of site-specific parsers                      |

---

## 📌 Output Sample

```text
001: title:The Shawshank Redemption                link:https://www.imdb.com/title/tt0111161/
002: title:The Godfather                           link:https://www.imdb.com/title/tt0068646/
...
```

---

## 🤝 Contributing

Feel free to submit issues, suggestions, or implement a new parser for your favorite movie site!
