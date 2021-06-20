use clap::{Clap, AppSettings};

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Paweł \"Inver\" Jankowski")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Main {
    input: String,
}

pub fn execute() {
    let command = Main::parse();

    println!("Hello {}", command.input);
}