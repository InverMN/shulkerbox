use std::env;
use common::run_webserver_process;

#[tokio::main]
async fn main() {
    if env::args().len() == 1 { 
        run_webserver_process();
    } else {
        cli::execute().await;
    };
}