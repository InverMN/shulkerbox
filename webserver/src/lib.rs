use std::process::exit;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "GUI will be served here!"
}

#[get("/api/v1/server/stop")]
fn server_stop() {
    exit(0);
}

pub async fn start() {
    rocket::build().mount("/", routes![index, server_stop]).launch().await.unwrap();
}