use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::{header, Client};
use url::Url;

use crate::tweet::{Tweet, TweetType};
use crate::utils::calc_token;

pub struct TweetFetcher {
    client: Client,
}

impl TweetFetcher {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self { client })
    }

    pub async fn fetch(&self, tweet_id: u64) -> Result<Option<Tweet>, reqwest::Error> {
        static ENDPOINT: Lazy<Url> =
            Lazy::new(|| Url::parse("https://cdn.syndication.twimg.com/tweet-result").unwrap());
        let mut url = ENDPOINT.clone();
        let token = calc_token(tweet_id);

        url.query_pairs_mut()
            .append_pair("id", &tweet_id.to_string())
            .append_pair("token", &token);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("*/*"));
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:122.0) Gecko/20100101 Firefox/122.0",
            ),
        );

        let tweet_type: TweetType = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        match tweet_type {
            TweetType::Tweet(tweet) => Ok(Some(tweet)),
            TweetType::TweetTombstone => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn photos() {
        let tweet_fetcher = TweetFetcher::new().unwrap();
        let tweet_id: u64 = 1079631553641164802;
        let tweet = tweet_fetcher.fetch(tweet_id).await.unwrap().unwrap();
        assert!(!tweet.text.is_empty());
        assert_eq!(tweet_id.to_string(), tweet.id_str);
        assert_eq!(3, tweet.photos.len());
        assert!(tweet.video.is_none());
    }

    #[tokio::test]
    async fn youtube_live_link() {
        let tweet_fetcher = TweetFetcher::new().unwrap();
        let tweet_id: u64 = 1753365478318416281;
        let tweet = tweet_fetcher.fetch(tweet_id).await.unwrap().unwrap();
        assert!(!tweet.text.is_empty());
        assert_eq!(tweet_id.to_string(), tweet.id_str);
        assert_eq!(0, tweet.photos.len());
        assert!(tweet.video.is_none());
    }

    #[tokio::test]
    async fn video() {
        let tweet_fetcher = TweetFetcher::new().unwrap();
        let tweet_id: u64 = 1727250580131750107;
        let tweet = tweet_fetcher.fetch(tweet_id).await.unwrap().unwrap();
        assert!(!tweet.text.is_empty());
        assert_eq!(tweet_id.to_string(), tweet.id_str);
        assert_eq!(0, tweet.photos.len());
        assert!(tweet.video.is_some());
    }

    #[tokio::test]
    async fn problematic() {
        let tweet_fetcher = TweetFetcher::new().unwrap();
        let tweet_id: u64 = 1733455117977112615;
        let tweet = tweet_fetcher.fetch(tweet_id).await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn tombstone() {
        let tweet_fetcher = TweetFetcher::new().unwrap();
        let tweet_id: u64 = 1878683993618718993;
        let tweet = tweet_fetcher.fetch(tweet_id).await.unwrap();
        assert!(tweet.is_none());
    }
}
