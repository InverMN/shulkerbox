use clap::{Clap, AppSettings};

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
        Subcommand::Start => { println!("START SERVER!") },
        Subcommand::Stop => { println!("STOP SERVER!") },
        Subcommand::Webserver(webserver) => {
            match webserver {
                Webserver::Start => { 
                    println!("Starting server...");
                    webserver::start().await;
                },
                Webserver::Stop => { 
                    println!("Stopping server...");
                    ureq::get("http://localhost:8000/api/v1/server/stop").call().unwrap();
                    println!("Server has stopped.");
                },
            }
        },
    };
}