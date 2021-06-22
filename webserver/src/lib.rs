mod state;

use std::{process::exit, };

use common::{read_config};
use rocket::{State, config::Config, log::LogLevel};
use state::HitCount;

use crate::state::InitStateExt;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "GUI will be served here!"
}

#[get("/api/v1/service/status")]
fn server_status() -> &'static str {
    "OK"
}

#[get("/api/v1/service/stop")]
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

#[get("/api/v1/server/create")]
fn create_server() -> &'static str {
    common::server::create();
    "Created"
}

#[get("/api/v1/hit")]
fn hit(hit_count: &State<HitCount>) -> &'static str {
    hit_count.increment();
    "OK"
}


#[get("/api/v1/hit/count")]
fn hit_count(hit_count: &State<HitCount>) -> String {
    hit_count.get()
}

pub async fn start() {
    let mut server_config = Config::default();
    server_config.log_level = LogLevel::Off;
    rocket::build()
        .init_app_state()
        .configure(server_config)
        .mount("/", routes![index, server_stop, config, server_status, mantain_files, create_server, hit_count, hit])
        .launch()
        .await
        .unwrap();
}