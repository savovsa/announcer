use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub audio_folder_path: String,
    pub messages: Messages,
}

impl Config {
    pub fn new() -> Config {
        Config {
            audio_folder_path: "sounds".to_string(),
            messages: HashMap::new(),
        }
    }

    pub fn get_path() -> PathBuf {
        PathBuf::from("announcer.json")
    }
}

/// The key is the audio file name
type Messages = HashMap<String, Message>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub display_name: String,
    pub volume: f32,
}

pub fn save_config(config: &Config, path: Option<&PathBuf>) {
    let default_path = Config::get_path();
    let path = path.unwrap_or(&default_path);

    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &config).unwrap();
}

pub fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let file_size = file.metadata()?.len();

    if file_size == 0 {
        return Err("The config file is empty.".into());
    }

    let reader = std::io::BufReader::new(file);

    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

pub mod endpoints {
    use rodio::Decoder;
    use std::{fs::File, io::BufReader, path::PathBuf};
    use surf::Body;
    use tide::Response;

    use crate::Request;

    pub async fn get_messages(req: Request) -> tide::Result {
        let mut res = Response::new(200);
        let state = &req.state().lock().unwrap();
        let body = Body::from_json(&state.config.lock().unwrap().messages)?;
        res.set_body(body);
        Ok(res)
    }

    pub async fn get_message(req: Request) -> tide::Result {
        let mut res = Response::new(200);

        let name: String = req.param("name")?.parse()?;
        let state = &req.state().lock().unwrap();
        let config = state.config.lock().unwrap();
        let value = config.messages.get(&name);

        let body = Body::from_json(&value)?;
        res.set_body(body);
        Ok(res)
    }

    pub async fn play_message(req: Request) -> tide::Result {
        let name: String = req.param("name")?.parse()?;
        let state = &req.state().lock().unwrap();
        let config = state.config.lock().unwrap();
        let message = config.messages.get(&name);

        if message == None {
            return Ok(Response::new(404));
        }

        let path = PathBuf::from(&config.audio_folder_path).join(name);
        let file = File::open(path);

        if file.is_err() {
            return Ok(Response::new(500));
        }

        let reader = BufReader::new(file.unwrap());
        let source = Decoder::new(reader).unwrap();

        let sink = state.sink.lock().unwrap();
        sink.append(source);
        sink.play();

        let res = Response::new(200);
        Ok(res)
    }

    pub async fn delete_message(req: Request) -> tide::Result {
        let mut res = Response::new(200);

        let name: String = req.param("name")?.parse()?;
        let state = &req.state().lock().unwrap();
        let mut config = state.config.lock().unwrap();
        let value = config.messages.remove(&name);

        let body = Body::from_json(&value)?;
        let path = PathBuf::from(&config.audio_folder_path).join(name);

        let file_exists = std::fs::metadata(path.clone()).is_ok();
        if (file_exists) {
            std::fs::remove_file(path)?;
        }

        res.set_body(body);
        Ok(res)
    }
}
