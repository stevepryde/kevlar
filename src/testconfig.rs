use chrono::Local;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{create_dir, create_dir_all, File};
use std::io::ErrorKind;
use std::path::PathBuf;

pub enum ConfigType {
    File(PathBuf),
    //    Env
}

#[derive(Serialize, Deserialize)]
pub struct TestConfig {
    pub path: PathBuf,
}

impl TestConfig {
    /// Load the config using one of the available methods.
    pub fn load(test_name: &str, config_type: ConfigType) -> Self {
        let mut config = match config_type {
            ConfigType::File(f) => TestConfig::load_from_file(&f),
        };

        config.path = TestConfig::create_unique_dir(&config.path, test_name);
        config
    }

    /// Load config from JSON file.
    fn load_from_file(config_path: &PathBuf) -> Self {
        let file = File::open(config_path).expect(&format!(
            "Error opening config file '{}'",
            config_path.display()
        ));
        let json: serde_json::Value = serde_json::from_reader(file).expect(&format!(
            "Error parsing config file '{}'",
            config_path.display()
        ));
        serde_json::from_value(json).expect(&format!(
            "Error validating config file '{}'",
            config_path.display()
        ))
    }

    /// Create a unique test workspace directory for the test to place files in.
    fn create_unique_dir(base_path: &PathBuf, test_name: &str) -> PathBuf {
        if !base_path.exists() {
            create_dir_all(base_path).expect(&format!(
                "Unable to create test workspace: {}",
                base_path.display()
            ));
        }

        let test_name_lower = test_name.to_lowercase();
        let test_name_base = Regex::new(r"[^a-z0-9]")
            .unwrap()
            .replace(&test_name_lower, "");
        assert!(test_name_base.len() > 0, "Invalid test name: {}", test_name);

        let mut count = 0;
        loop {
            let mut new_dir = base_path.clone();
            let now_str = Local::now().format("&Y&m&d_&H&M&S");
            let try_name = if count > 0 {
                format!("{}_{}_{}", test_name_base, now_str, count)
            } else {
                format!("{}_{}", test_name_base, now_str)
            };

            new_dir.push(try_name);
            match create_dir(new_dir.clone()) {
                Ok(_) => return new_dir,
                Err(e) => match e.kind() {
                    ErrorKind::AlreadyExists => {}
                    _ => panic!("Error creating directory '{}': {:?}", new_dir.display(), e),
                },
            }

            count += 1;
        }
    }
}
