use std::sync::{Arc, Mutex};

use announcer::messages::*;
use surf::Url;
use tide::http::{Method, Request};
#[test]
fn load_config_from_file() {
    let path = std::path::PathBuf::from("tests/messages_test_config.json");
    let Config {
        audio_folder_path,
        messages,
    } = load_config(&path).unwrap();

    assert_eq!(audio_folder_path, "sounds/");

    let message = messages.get("sound.mp3").unwrap();

    assert_eq!(message.display_name, "Sound 1");
}

#[test]
fn save_config_to_file() {
    let config = Config {
        audio_folder_path: "sounds/".to_string(),
        messages: [(
            "sound2.mp3".to_string(),
            Message {
                display_name: "Sound 2".to_string(),
                volume: 60_f32,
            },
        )]
        .iter()
        .cloned()
        .collect(),
    };

    let path = std::path::PathBuf::from("tests/messages_test_config_saved.json");

    save_config(&config, Some(&path));

    let Config {
        audio_folder_path,
        messages,
    } = load_config(&path).unwrap();

    assert_eq!(audio_folder_path, "sounds/");

    let message = messages.get("sound2.mp3").unwrap();

    assert_eq!(message.display_name, "Sound 2");
}

#[test]
fn plays_massage_if_it_exists_in_configuration() {
    let config = Config {
        audio_folder_path: "sounds/".to_string(),
        messages: [(
            "sound2.mp3".to_string(),
            Message {
                display_name: "Sound 2".to_string(),
                volume: 60_f32,
            },
        )]
        .iter()
        .cloned()
        .collect(),
    };
    let state = Arc::new(Mutex::new(config));
    let http_request = Request::new(
        Method::Get,
        Url::parse("http://0.0.0.0/play/sound2.mp3").unwrap(),
    );

    // this is private, how do I fix that?
    let params = vec![tide::request::Params::new()];

    let req = announcer::Request::new(state, http_request, params);

    let result = announcer::messages::endpoints::play_message(req);
}

#[test]
fn does_not_play_massage_if_its_not_in_the_configuration() {}
