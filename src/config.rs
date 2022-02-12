use crate::gh::get_logged_in_user;
use serde_derive::{Deserialize, Serialize};
use std::fs::{create_dir_all, metadata, read_to_string, write};
use std::io::stdin;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub org: String,
    pub provider: Option<String>,
}

impl Config {
    pub fn new(org: Option<String>, provider: Option<String>) -> Config {
        let unwrapped_org = org.unwrap_or(get_logged_in_user());
        Config {
            org: unwrapped_org,
            provider: provider,
        }
    }
    pub fn save(&self) -> std::io::Result<()> {
        let config_path = PathBuf::from(".config");
        let config_file = &config_path.join("gh-tf-mod.yaml");
        let config_string = serde_yaml::to_string(self).expect("Could not serialize config");
        if metadata(&config_path).is_err() {
            create_dir_all(&config_path)?;
        }
        if metadata(&config_path).is_ok() {
            println!(
                "File {} already exists. Please type \"yes\" to confirm.",
                &config_file.to_string_lossy()
            );
            let mut buf = String::new();
            stdin()
                .read_line(&mut buf)
                .expect("Failed to read response.");
            let confirmation = buf.lines().next().expect("Could not read entry.");
            if confirmation == "yes" {
                write(config_file, &config_string)?;
            } else {
                println!("Aborting.");
            }
        }
        Ok(())
    }
    pub fn load(org: &Option<String>, provider: &Option<String>) -> Config {
        let config_path = PathBuf::from(".config");
        let config_file = &config_path.join("gh-tf-mod.yaml");
        if metadata(&config_path).is_ok() {
            let config_string = read_to_string(config_file).expect("Could not read config");
            let config: Config =
                serde_yaml::from_str(&config_string).expect("Could not deserialize config");
            config
        } else {
            Config::new(org.clone(), provider.clone())
        }
    }
}
