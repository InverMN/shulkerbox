use std::{convert::TryInto, fs::{copy, create_dir_all}, io::Result, path::Path, process::Command, thread};

use downloader::{Download, Downloader};
use log::debug;
use nanoid::nanoid;

use crate::files::{mantain_file_structure, app_directory};

fn get_id_alphabet() -> [char; 62] {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars();
    chars.collect::<Vec<char>>().try_into().unwrap_or_else(|v: Vec<char>| panic!("Expected a Vec of length {} but it was {}", 62, v.len()))
}

fn generate_server_id() -> String {
    nanoid!(3, &get_id_alphabet())
}

fn create_server_directory(server_directory: &str) -> Result<()> {
    create_dir_all(server_directory)?;
    Ok(())
}

fn copy_default_files(server_directory: &str) -> Result<()> {
    let eula_source_path = format!("{}/defaults/eula.txt", app_directory());
    let server_properties_source_path = format!("{}/defaults/server.properties", app_directory());
    let eula_target_path = format!("{}/eula.txt", server_directory);
    let server_properties_target_path = format!("{}/server.properties", server_directory);

    println!("Copying file from {} to {}", eula_source_path, eula_target_path);
    copy(eula_source_path, eula_target_path)?;
    println!("Copying file from {} to {}", server_properties_source_path, server_properties_target_path);
    copy(server_properties_source_path, server_properties_target_path)?;

    println!("Default files copied");
    Ok(())
}

fn download_server_engine(server_directory: &str) -> Result<()> {
    let server_directory = String::from(server_directory);
    thread::spawn(move || {
        let downloaded_file = Download::new("https://papermc.io/api/v2/projects/paper/versions/1.16.5/builds/778/downloads/paper-1.16.5-778.jar");
        let mut downloader = Downloader::builder()
            .download_folder(Path::new(&server_directory))
            .build()
            .unwrap();
        downloader.download(&[downloaded_file]).unwrap();
    }).join().unwrap();

    Ok(())
}

pub fn create() {
    let server_id = generate_server_id();
    let server_directory = format!("{}/servers/{}", app_directory(), server_id);
    println!("Server directory: {}", server_directory);
    
    mantain_file_structure().unwrap();
    create_server_directory(&server_directory).unwrap();
    copy_default_files(&server_directory).unwrap();
    download_server_engine(&server_directory).unwrap();
}