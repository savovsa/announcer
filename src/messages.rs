use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize)]
pub struct Config<'a> {
    pub audio_folder_path: &'a str,
    pub messages: Messages<'a>,
}

type Messages<'a> = HashMap<&'a str, Message<'a>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<'a> {
    pub name: &'a str,
    pub volume: f32,
}

pub fn save_config(config: Config) {
    let file = std::fs::File::create("config.json").unwrap();
    serde_json::to_writer_pretty(file, &config).unwrap();
}
