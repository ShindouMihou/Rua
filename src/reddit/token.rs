use cached::proc_macro::cached;
use chrono::Local;
use reqwest::Error;
use crate::models::TokenResponse;

#[cached(time = 3600, result = true, sync_writes = true)]
pub async fn request() -> Result<String, Error> {
    let response = reqwest::Client::new()
        .post("https://www.reddit.com/api/v1/access_token")
        .header("User-Agent", dotenv!("REDDIT_USER_AGENT"))
        .header("Accept", "*/*")
        .header("Cache-Control", "no-cache")
        .header("Host", "www.reddit.com")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept-Encoding", "gzip, deflate")
        .multipart(reqwest::multipart::Form::new().text("grant_type", "client_credentials"))
        .basic_auth(dotenv!("REDDIT_ID"), Some(dotenv!("REDDIT_SECRET")))
        .send()
        .await?;

    let token = response.json::<TokenResponse>().await?;
    println!("{} : [event: generate_token, token={}]", Local::now().to_string(), &token.access_token);
    Ok(token.access_token)
}
