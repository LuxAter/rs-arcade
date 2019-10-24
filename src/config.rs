use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub window: WindowSettings,
    source_file: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WindowSettings {
    pub res: Vec<f32>,
    pub fullscreen: Option<bool>,
    pub borderless: Option<bool>,
    pub resizable: Option<bool>,
}

impl Config {
    pub fn set_source(&mut self, file: String) {
        self.source_file = Some(file);
    }
    pub fn write(&mut self) -> Result<(), std::io::Error> {
        println!("{:?}", self);
        println!("{:?}", &*self);
        let config_str = toml::to_string(&*self).expect("Failed to serialize config");
        fs::write(&self.source_file.as_ref().unwrap(), config_str)
    }
}

pub fn load(file: &str) -> Config {
    let contents = fs::read_to_string(file).expect("error while reading file");
    let mut config: Config = toml::from_str(&contents).expect("error while parsing toml");
    config.set_source(file.to_string());
    return config;
}
