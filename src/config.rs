use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Read
};
use tracing::log::info;

// TODO: Can I have one impl for all of these?
// TODO: Create a baseline template via code (Such as if the toml file does not exist)

/// Structure for `config.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub accent_colour: [u8; 3],
    pub e621_blacklist: String,
    pub e621_useragent: String
}

impl Config {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub fn get(path: &str) -> Config {
        let mut file_opened = match File::open(path) {
            Ok(file_opened) => file_opened,
            Err(err) => panic!("Error opening toml file: {err}")
        };

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents) {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(err) => panic!("Error reading toml file: {err}")
        }

        return match toml::from_str::<Config>(&contents) {
            Ok(secrets) => secrets,
            Err(err) => panic!("Error serialising toml file: {err}")
        };
    }

    /// Write the struct to a toml file
    pub fn write(new_data: &Config, path: &str) {
        let struct_to_toml_string = match toml::to_string_pretty(&new_data) {
            Ok(string) => string,
            Err(err) => panic!("Error serialising struct to toml string: {err}")
        };

        match fs::write(path, struct_to_toml_string) {
            Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
            Err(err) => panic!("Error writing toml file: {err}")
        }
    }

    /// Mutate the struct
    pub fn reload(&mut self, new_data: &Config) {
        self.accent_colour = new_data.accent_colour;
        self.e621_blacklist = new_data.e621_blacklist.clone();
        self.e621_useragent = new_data.e621_useragent.clone();
    }
}
/// Structure for `heck.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Heck {
    pub heck: Vec<String>
}

impl Heck {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub fn get(path: &str) -> Heck {
        let mut file_opened = match File::open(path) {
            Ok(file_opened) => file_opened,
            Err(err) => panic!("Error opening toml file: {err}")
        };

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents) {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(err) => panic!("Error reading toml file: {err}")
        }

        return match toml::from_str::<Heck>(&contents) {
            Ok(secrets) => secrets,
            Err(err) => panic!("Error serialising toml file: {err}")
        };
    }

    /// Write the struct to a toml file
    pub fn write(new_data: &Heck, path: &str) {
        let struct_to_toml_string = match toml::to_string_pretty(&new_data) {
            Ok(string) => string,
            Err(err) => panic!("Error serialising struct to toml string: {err}")
        };

        match fs::write(path, struct_to_toml_string) {
            Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
            Err(err) => panic!("Error writing toml file: {err}")
        }
    }

    /// Mutate the struct
    pub fn reload(&mut self, new_data: &Heck) {
        self.heck = new_data.heck.clone();
    }
}

/// Structure for `quotes.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Quotes {
    pub quotes: Vec<Quote>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Quote {
    pub user_id: u64,
    pub quote: String
}

impl Quotes {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub fn get(path: &str) -> Quotes {
        let mut file_opened = match File::open(path) {
            Ok(file_opened) => file_opened,
            Err(err) => panic!("Error opening toml file: {err}")
        };

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents) {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(err) => panic!("Error reading toml file: {err}")
        }

        return match toml::from_str::<Quotes>(&contents) {
            Ok(secrets) => secrets,
            Err(err) => panic!("Error serialising toml file: {err}")
        };
    }

    /// Write the struct to a toml file
    pub fn write(new_data: &Quotes, path: &str) {
        let struct_to_toml_string = match toml::to_string_pretty(&new_data) {
            Ok(string) => string,
            Err(err) => panic!("Error serialising struct to toml string: {err}")
        };

        match fs::write(path, struct_to_toml_string) {
            Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
            Err(err) => panic!("Error writing toml file: {err}")
        }
    }

    /// Mutate the struct
    pub fn reload(&mut self, new_data: &Quotes) {
        self.quotes = new_data.quotes.clone();
    }
}

/// Structure for `secrets.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Secrets {
    pub discord_token: Option<String>,
    pub e621_token: Option<String>,
    pub twitter_api: Option<String>,
    pub furaffinity_cookies: [String; 2]
}

impl Secrets {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub fn get(path: &str) -> Secrets {
        let mut file_opened = match File::open(path) {
            Ok(file_opened) => file_opened,
            Err(err) => panic!("Error opening toml file: {err}")
        };

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents) {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(err) => panic!("Error reading toml file: {err}")
        }

        return match toml::from_str::<Secrets>(&contents) {
            Ok(secrets) => secrets,
            Err(err) => panic!("Error serialising toml file: {err}")
        };
    }

    /// Write the struct to a toml file
    pub fn write(new_data: &Secrets, path: &str) {
        let struct_to_toml_string = match toml::to_string_pretty(&new_data) {
            Ok(string) => string,
            Err(err) => panic!("Error serialising struct to toml string: {err}")
        };

        match fs::write(path, struct_to_toml_string) {
            Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
            Err(err) => panic!("Error writing toml file: {err}")
        }
    }

    /// Mutate the struct
    pub fn reload(&mut self, new_data: &Secrets) {
        self.discord_token = new_data.discord_token.clone();
        self.e621_token = new_data.e621_token.clone();
        self.twitter_api = new_data.twitter_api.clone();
    }
}

/// Structure for `stories.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stories {
    pub stories: Vec<[String; 2]>
}

impl Stories {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub fn get(path: &str) -> Stories {
        let mut file_opened = match File::open(path) {
            Ok(file_opened) => file_opened,
            Err(err) => panic!("Error opening toml file: {err}")
        };

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents) {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(err) => panic!("Error reading toml file: {err}")
        }

        return match toml::from_str::<Stories>(&contents) {
            Ok(secrets) => secrets,
            Err(err) => panic!("Error serialising toml file: {err}")
        };
    }

    /// Write the struct to a toml file
    pub fn write(new_data: &Stories, path: &str) {
        let struct_to_toml_string = match toml::to_string_pretty(&new_data) {
            Ok(string) => string,
            Err(err) => panic!("Error serialising struct to toml string: {err}")
        };

        match fs::write(path, struct_to_toml_string) {
            Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
            Err(err) => panic!("Error writing toml file: {err}")
        }
    }

    /// Mutate the struct
    pub fn reload(&mut self, new_data: &Stories) {
        self.stories = new_data.stories.clone();
    }
}
