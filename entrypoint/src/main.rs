use std::env::{self, current_exe};

#[tokio::main]
async fn main() {
    if env::args().into_iter().skip(1).collect::<Vec<String>>().join(" ") == "webserver start" {
        webserver::start().await;
    } else if env::args().len() == 1 { 
        run_webserver_process()
    } else {
        cli::execute();
    }
}

fn run_webserver_process() {
    use std::process::Command;

    let executables = current_exe().unwrap().into_os_string().into_string().unwrap();
    let statement = format!("{} webserver start", executables);
    
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(statement)
            .spawn()
            .unwrap();
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(statement)
            .spawn()
            .unwrap();
    }
}