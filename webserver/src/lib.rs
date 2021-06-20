use std::{process::exit, };

use common::{read_config};
use rocket::{config::Config, log::LogLevel};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "GUI will be served here!"
}

#[get("/api/v1/server/status")]
fn server_status() -> &'static str {
    "OK"
}

#[get("/api/v1/server/stop")]
fn server_stop() -> &'static str {
    use std::{thread::{sleep, spawn}, time::Duration};
    spawn(|| {
        sleep(Duration::from_millis(100));
        println!("Shutting server");
        exit(0);
    });
    println!("Recieved exit server route");
    "OK"
}

#[get("/api/v1/config")]
fn config() -> String {
    serde_json::to_string(&read_config()).unwrap()
}

#[get("/api/v1/files/mantain")]
fn mantain_files() -> &'static str {
    common::files::mantain_file_structure().unwrap();
    "OK"
}

pub async fn start() {
    let mut server_config = Config::default();
    server_config.log_level = LogLevel::Off;
    rocket::build().configure(server_config).mount("/", routes![index, server_stop, config, server_status, mantain_files]).launch().await.unwrap();
}