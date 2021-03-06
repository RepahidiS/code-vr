mod window;
mod graphics;
mod sound;

use std::fs::OpenOptions;
use std::io::{Read, Write};

pub use self::window::WindowConfig;
pub use self::graphics::GraphicsConfig;
pub use self::sound::SoundConfig;

use std::fs;
use std::env;

use serde_json;

/// Configuration object passed to the renderer.
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  pub window: WindowConfig,
  pub graphics: GraphicsConfig,
  pub sound: SoundConfig
}

impl Config {
    fn new() -> Config {
        Config {
            window: WindowConfig {
                display: 0,
                resolution: [0, 0],
                fullscreen: true
            },
            graphics: GraphicsConfig {
                antialiasing: 0,
                vsync: false
            },
            sound: SoundConfig {
                master: 100.0
            }
        }
    }
}

/// Tries to read Config JSON data from the working directory, returns either defaults or what was in file.
pub fn read() -> Config {

    let default_config = Config::new();

    // Create codevr/ folder in WORKING_DIRECTORY
    let mut working = env::var("APPDATA").unwrap();
    working.push_str("/codevr");
    fs::create_dir(working.as_str()).unwrap_or_default();

    working.push_str("/config.json");


    let open = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(working.as_str());

    let mut contents = String::new();

    let mut file = match open {
        Err(_) => return default_config,
        Ok(file) => file,
    };

    match file.read_to_string(&mut contents) {
        Err(_) => return default_config,
        Ok(_) => (),
    }

    if contents.is_empty() {

        contents.insert_str(0, serde_json::to_string_pretty(&default_config).unwrap().as_str());

        match file.write_all(contents.as_bytes()) {

            Err(_) => return default_config,
            Ok(_) => (),
        }
    }

    serde_json::from_str(contents.as_str()).unwrap()
}