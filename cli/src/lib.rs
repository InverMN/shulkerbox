use std::time::Duration;

use clap::{Clap, AppSettings};
use common::run_webserver_process;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Paweł \"Inver\" Jankowski", name = "shulker")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Main {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    Stop,
    Start,
    Webserver(Webserver),
}

#[derive(Clap)]
enum Webserver {
    Start,
    Stop,
}

pub async fn execute() {
    let command = Main::parse();

    match command.subcommand {
        Subcommand::Start => {
            let response = ureq::get("http://localhost:8000/api/v1/service/status").timeout(Duration::from_secs(2)).call();
            match response {
                Ok(_) => {
                    println!("Shulker is already running at port 8000");
                },
                Err(_) => {
                    println!("Shulker has started on port 8000");
                    run_webserver_process();
                },
            };
        },
        Subcommand::Stop => { 
            ureq::get("http://localhost:8000/api/v1/service/stop").call().unwrap();
            println!("Shulker has stopped");
        },
        Subcommand::Webserver(webserver) => {
            match webserver {
                Webserver::Start => { 
                    let response = ureq::get("http://localhost:8000/api/v1/service/status").timeout(Duration::from_secs(2)).call();
                    match response {
                        Ok(_) => {
                            println!("Shulker is already running at port 8000");
                        },
                        Err(_) => {
                            println!("Shulker has started on port 8000");
                            webserver::start().await;
                        },
                    };
                },
                Webserver::Stop => { 
                    let response = ureq::get("http://localhost:8000/api/v1/service/status").timeout(Duration::from_secs(2)).call();
                    match response {
                        Ok(_) => {
                            ureq::get("http://localhost:8000/api/v1/service/stop").call().unwrap();
                            println!("Server has stopped.");
                        },
                        Err(_) => {
                            println!("No server running");
                        },
                    };
                    
                },
            }
        },
    };
}