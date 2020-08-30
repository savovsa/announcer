use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub message: String,
}

pub fn save_config(config: Config) {
    let file = std::fs::File::create("config.json").unwrap();
    serde_json::to_writer_pretty(file, &config);
}
