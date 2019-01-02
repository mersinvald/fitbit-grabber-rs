use directories::ProjectDirs;
use failure::{format_err, Error};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub fitbit: Option<FitbitConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FitbitConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

impl FitbitConfig {
    fn from_env_vars(client_id: &str, client_secret: &str) -> FitbitConfig {
        let mut fc = FitbitConfig {
            client_id: None,
            client_secret: None,
        };

        if let Ok(id) = env::var(client_id) {
            fc.client_id = Some(id);
        };
        if let Ok(secret) = env::var(client_secret) {
            fc.client_secret = Some(secret);
        };
        fc
    }
}

impl Config {
    /// Load a config from the environment. A config object will be constructed
    /// from a combination of environment variables and/or config files on disk.
    /// Environment variables supercede values in files.
    pub fn load(path: Option<&str>) -> Result<Config, Error> {
        let mut conf = Config { fitbit: None };

        match path {
            Some(path) => {
                // custom path to config file, passed as a flag
                if let Ok(found) = Config::from_toml_file(path) {
                    conf = found;
                }
            }
            None => {
                let project_dirs = ProjectDirs::from("", "", "fitbit-grabber")
                    .ok_or_else(|| format_err!("app dirs do not exist"))?;
                // default path
                let config_path = project_dirs.config_dir().join("conf.toml");
                if let Ok(found) = Config::from_toml_file(config_path) {
                    conf = found;
                }
            }
        }
        let temp = FitbitConfig::from_env_vars("FITBIT_CLIENT_ID", "FITBIT_CLIENT_SECRET");
        if conf.fitbit.is_none() {
            conf.fitbit = Some(temp);
        }
        Ok(conf)
    }

    /// Deserialize a config from a toml file without applying environment variables.
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut f = File::open(path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        Ok(toml::from_str::<Config>(buffer.as_str())?)
    }
}
