use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RuaPost {
    data: RuaMetadata,
}

#[derive(Deserialize, Serialize)]
pub struct RuaMetadata {
    id: String,
    title: String,
    #[serde(rename(deserialize = "url_overridden_by_dest", serialize = "image"))]
    image: Option<String>,
    ups: i32,
    downs: i32,
    #[serde(rename = "over_18")]
    nsfw: bool,
    author: String,
}

#[derive(Deserialize, Serialize)]
pub struct SubredditResponse {
    pub(crate) data: SubredditInnerResponse,
}

#[derive(Deserialize, Serialize)]
pub struct SubredditInnerResponse {
    pub(crate) children: Vec<RuaPost>,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub(crate) access_token: String,
}