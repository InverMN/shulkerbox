mod state;
mod config;
mod routing;

use routing::AppRoutingExt;
use config::AppConfigExt;
use state::InitStateExt;

#[macro_use] extern crate rocket;

pub async fn start() {
    rocket::build()
        .init_app_state()
        .impl_app_config()
        .attach_app_routes()
        .launch()
        .await
        .unwrap();
}