mod utils;
mod fetch;
pub mod tweet;

pub use fetch::TweetFetcher;
pub use reqwest::Error as TweetFetcherError;
