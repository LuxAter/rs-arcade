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
        let config_str = serde_json::to_string(&*self).unwrap();
        fs::write(&self.source_file.as_ref().unwrap(), config_str)
    }
}

pub fn load(file: &str) -> Config {
    match fs::read_to_string(file) {
        Ok(v) => {
            let mut config: Config = serde_json::from_str(&v).expect("error while parsing json");
            config.set_source(file.to_string());
            return config;
        }
        Err(_) => {
            return Config {
                window: WindowSettings {
                    res: vec![500.0, 500.0],
                    fullscreen: Some(false),
                    borderless: Some(true),
                    resizable: Some(true),
                },
                source_file: Some(file.to_string()),
            };
        }
    }
}
