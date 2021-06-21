use std::env::var;
use std::fs::*;
use std::io::Result;
use crate::copy_file;

pub fn app_directory() -> String {
    format!("{}/.shulkerbox", var("HOME").unwrap())
}

pub fn mantain_file_structure() -> Result<()> {
    create_dirs("servers")?;
    create_dirs("installers")?;
    create_dirs("defaults")?;
    copy_file!("../../resources/eula.txt", "defaults/eula.txt");
    copy_file!("../../resources/server.properties", "defaults/server.properties");

    Ok(())
}

pub fn create_dirs(relative_path: &str) -> Result<()> {
    create_dir_all(format!("{}/{}", app_directory(), relative_path))?;
    Ok(())
}

#[macro_export]
macro_rules! copy_file {
    ($source_path: expr, $target_path: expr) => {
        let _result: ::std::io::Result<()> = match ::std::fs::metadata($target_path) {
            Ok(_) => Ok(()),
            Err(_) => {
                let content = include_str!($source_path);
                ::std::fs::write(format!("{}/{}", app_directory(), $target_path), content)?;
                Ok(())
            },
        };
    };
}