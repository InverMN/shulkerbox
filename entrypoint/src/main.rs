use std::env;
use common::run_webserver_process;
use log4rs::{
    append::console::ConsoleAppender,
    encode::pattern::PatternEncoder,
    config::{Appender, Config, Logger, Root},
};
use log::LevelFilter;

#[tokio::main]
async fn main() {
    init_logger();

    if env::args().len() == 1 { 
        run_webserver_process();
    } else {
        cli::execute().await;
    };
}

fn init_logger() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d}: {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("app::shulkerbox", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}