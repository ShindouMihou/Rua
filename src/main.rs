// This is disabled because I am too lazy to change crate name.
#![allow(non_snake_case)]

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use actix_web::middleware::Logger;
use chrono::Local;
use colored::Colorize;
use rand::seq::{SliceRandom};

#[path = "reddit/subreddit.rs"]
mod subreddit;
#[path = "reddit/token.rs"]
mod token;
#[path = "models/model_factory.rs"]
mod models;
#[path = "internals/env.rs"]
mod env;

#[get("/get/{subreddit}/")]
async fn get_subreddit_single(
    request: HttpRequest,
    subreddit: web::Path<String>,
) -> impl Responder {
    println!(
        "{} : [event=get_single, ip={}, subreddit={}]",
        Local::now().to_string(),
        request.connection_info().realip_remote_addr().unwrap_or("???"),
        &subreddit,
    );
    HttpResponse::Ok().json(
        subreddit::serialize_from_json(subreddit::get_posts(subreddit.to_string()).await.unwrap())
            .choose(&mut rand::thread_rng()),
    )
}

#[get("/")]
async fn get_index(
    request: HttpRequest
) -> impl Responder {
    println!(
        "{}: [event=get_index, ip={}]",
        Local::now().to_string(),
        request.connection_info().realip_remote_addr().unwrap_or("???"),
    );

    HttpResponse::Ok().content_type("application/json")
        .body("{\"message\":\"Rua, a simplified Reddit aggregator.\",\"github\":\"https://github.com/ShindouMihou/Rua\",\"endpoint\":\"/get/{subreddit}\"")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(
        "{} - Simplified Reddit Aggregation",
        "Rua".bold().bright_red()
    );
    println!(
        "You can now start requesting at configured endpoint: {}",
        env::get("SERVER_BIND").unwrap()
    );

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).wrap(Logger::new(
            "Received request from %a that took %D with response status %s. [time=%t]",
        )).service(get_subreddit_single).service(get_index)
    })
    .bind(env::get("SERVER_BIND").unwrap())?
    .run()
    .await
}
