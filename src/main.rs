// This is disabled because I am too lazy to change crate name.
#![allow(non_snake_case)]

#[macro_use]
extern crate dotenv_codegen;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use actix_web::middleware::Logger;
use chrono::Local;
use colored::Colorize;
use rand::seq::{SliceRandom};
use crate::env::get;

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
        )).service(get_subreddit_single)
    })
    .bind(dotenv!("SERVER_BIND"))?
    .run()
    .await
}
