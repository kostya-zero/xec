use std::{fs, collections::HashMap};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigModel {
    pub scriptlets: HashMap<String, Vec<String>>
}

pub struct ConfigManager;
impl ConfigManager {
    pub fn check() -> bool {
        if !Path::new("xec.conf").exists() {
            return false;
        }

        return true;
    }

    pub fn get_config() -> ConfigModel {
        let content: String = fs::read_to_string("xec.conf").expect("Failed to read data from file.");
        let cfg: ConfigModel = serde_json::from_str(content.as_str()).expect("Configuration file has bad JSON structure.");
        return cfg;
    }

    pub fn write_config(config: ConfigModel) {
        let cfg = serde_json::to_string(&config).unwrap();
        fs::write("xec.conf", cfg);
    }
}
