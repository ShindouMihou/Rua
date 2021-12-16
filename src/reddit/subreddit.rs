use cached::proc_macro::cached;
use chrono::Local;
use reqwest::Error;
use crate::models::{RuaPost, SubredditResponse};

use crate::token::request;
use crate::env::get;

pub fn serialize_from_json(json: String) -> Vec<RuaPost> {
    serde_json::from_str::<SubredditResponse>(&*json)
        .unwrap()
        .data
        .children
}

/// To get around the issue with caching, we are saving the
/// results as a string instead and serializing them from
/// the response.
#[cached(time = 3600, result = true, sync_writes = true)]
pub async fn get_posts(subreddit: String) -> Result<String, Error> {
    let token = request().await?;

    let response = reqwest::Client::new()
        .get(format!(
            "https://oauth.reddit.com/r/{}/hot.json?limit=100",
            &subreddit
        ))
        .header("User-Agent", env::get("REDDIT_USER_AGENT").unwrap())
        .header("Accept", "*/*")
        .header("Cache-Control", "no-cache")
        .header("Host", "oauth.reddit.com")
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();

    println!("{} : [event: fresh_subreddit, subreddit={}]", Local::now().to_string(), &subreddit);
    Ok(response.text().await?)
}
