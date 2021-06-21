pub mod config;
pub mod files;
pub use config::{Config, read_config};
pub mod server;

pub fn run_webserver_process() {
    use std::{process::{Command, exit, Stdio}, env::current_exe};

    let executables = current_exe().unwrap().into_os_string().into_string().unwrap();
    let statement = format!("{} webserver start", executables);
    
    let mut command = 
        if cfg!(target_os = "windows") {
            let mut header = Command::new("cmd");
            header.arg("/C");
            header
        } else {
            let mut header = Command::new("sh");
            header.arg("-c");
            header
        };

    command
        .arg(statement)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .unwrap();

    println!("Server starting on port 8000...");
    exit(0);
}