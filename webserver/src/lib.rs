use std::process::exit;

use common::{read_config};
use rocket::{config::Config, log::LogLevel, response::{content, status}};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "GUI will be served here!"
}

#[get("/api/v1/server/stop")]
fn server_stop() {
    exit(0);
}

#[get("/api/v1/config")]
fn config() -> String {
    serde_json::to_string(&read_config()).unwrap()
}

pub async fn start() {
    let mut server_config = Config::default();
    server_config.log_level = LogLevel::Off;
    rocket::build().mount("/", routes![index, server_stop, config]).launch().await.unwrap();
}