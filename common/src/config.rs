use serde::{Serialize, Deserialize};
use confy::load;
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
        }
    }
}

pub fn read_config() -> Config {
    load("shulkerbox").unwrap()
}