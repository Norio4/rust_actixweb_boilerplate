use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    funds: Vec<HashMap<String, String>>,
    cors: Vec<String>,
}

impl AppConfig {}

#[derive(Clone, Debug)]
pub struct ConfigInitializer {
    pub config_file: String,
}

impl ConfigInitializer {
    fn load_yaml(&self, path: &str) -> AppConfig {
        let f = fs::read_to_string(path);
        let s = f.unwrap();
        serde_yaml::from_str(&s).unwrap()
    }
    pub fn get_cors_origins(&self) -> Vec<String> {
        let config = &self.load_yaml(&self.config_file);
        let mut vec = Vec::new();
        for origin in config.cors.iter() {
            vec.push(origin.to_string());
        }
        vec
    }
    pub fn initialize(&self) -> AppConfig {
        let app_config = &self.load_yaml(&self.config_file);
        app_config.clone()
    }
}
