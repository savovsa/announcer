use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub audio_folder_path: String,
    pub messages: Messages,
}

/// The key is the audio file name
type Messages = HashMap<String, Message>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub display_name: String,
    pub volume: f32,
}

pub fn save_config(config: Config, path: &str) {
    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &config).unwrap();
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;

    // There is a panic if this is uncommented
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    println!("{}", contents);

    let reader = std::io::BufReader::new(file);

    // The panic happens here with the following error
    // Error("EOF while parsing a value", line: 1, column: 0)
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}
