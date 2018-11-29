use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: Option<String>
}

impl Config {
    fn empty() -> Config {
        Config { port: None }
    }

    fn file_name() -> Option<std::path::PathBuf> {
        match dirs::home_dir() {
            Some(path) => {
                let mut new_path = path.clone();
                new_path.push(".everdrivemd");
                Some(new_path)
            }
            _ => None
        }
    }

    fn read_config_file () -> Option<String> {
        match Config::file_name() {
            Some(name) => {
                let mut file = std::fs::File::open(name).ok()?;
                let mut contents = String::new();
                file.read_to_string(&mut contents).ok()?;
                Some(contents)
            }
            _ => None
        }
    }

    fn from_str(contents: String) -> Option<Config> {
        toml::from_str(&contents).ok()
    }

    pub fn read() -> Config {
        Config::read_config_file().and_then(Config::from_str).unwrap_or(Config::empty())
    }
}
