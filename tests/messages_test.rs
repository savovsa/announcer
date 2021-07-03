use std::path::PathBuf;

use announcer::messages::*;
use tide::http::{Method, Request, Response, Url};

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

#[async_std::test]
async fn plays_message_if_it_exists_in_configuration() {
    let sound_name = "soft-bells.mp3";
    let audio_folder = PathBuf::from("sounds");

    // Temporarily copy an audio file in the sounds folder,
    // because the play endpoint expects to be load a file.
    let new_audio_file = audio_folder.join(sound_name);
    let existing_audio_file = PathBuf::from("tests").join(sound_name);
    std::fs::copy(existing_audio_file, &new_audio_file).unwrap();

    let config = Config {
        audio_folder_path: audio_folder.to_str().unwrap().into(),
        messages: [(
            sound_name.into(),
            Message {
                display_name: "Hello".to_string(),
                volume: 1_f32,
            },
        )]
        .iter()
        .cloned()
        .collect(),
    };

    let app_with_state = announcer::create_app(Some(config), None).unwrap();

    let url_string = format!("https://example.com/play/{}", sound_name);
    let req = Request::new(Method::Get, Url::parse(&url_string).unwrap());
    let res: Response = app_with_state.app.respond(req).await.unwrap();

    // Clean up temp audio file
    std::fs::remove_file(new_audio_file).unwrap();

    assert_eq!(res.status(), 200);
}

#[async_std::test]
async fn does_not_play_message_if_its_not_in_the_configuration() {
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

    let app_with_state = announcer::create_app(Some(config), None).unwrap();

    let req = Request::new(
        Method::Get,
        Url::parse("https://example.com/play/sound0.mp3").unwrap(),
    );
    let res: Response = app_with_state.app.respond(req).await.unwrap();

    assert_eq!(res.status(), 404);
}

#[async_std::test]
async fn delete_message_from_config() {
    let message = Message {
        display_name: "Sound 2".to_string(),
        volume: 60_f32,
    };

    let config = Config {
        audio_folder_path: "sounds/".to_string(),
        messages: [(
            "sound2.mp3".to_string(),
            message.clone(),
        )]
        .iter()
        .cloned()
        .collect(),
    };

    let app_with_state = announcer::create_app(Some(config), None).unwrap();

    let req = Request::new(
        Method::Delete,
        Url::parse("https://example.com/message/sound2.mp3").unwrap(),
    );
    let mut res: Response = app_with_state.app.respond(req).await.unwrap();

    assert_eq!(res.status(), 200);

    let body: Message = res.body_json().await.unwrap();
    k9::assert_equal!(body, message);
    

    let req = Request::new(
        Method::Get,
        Url::parse("https://example.com/message/sound2.mp3").unwrap(),
    );
    let mut res: Response = app_with_state.app.respond(req).await.unwrap();
    let message: Option<Message> = res.body_json().await.unwrap();
    
    k9::assert_equal!(message, None);
}

#[async_std::test]
async fn delete_message_audio_file() {
    let sound_name = "soft-bells.mp3";
    let audio_folder = PathBuf::from("sounds");

    // Temporarily copy an audio file in the sounds folder,
    // because the play endpoint expects to be load a file.
    let new_audio_file = audio_folder.join(sound_name);
    let existing_audio_file = PathBuf::from("tests").join(sound_name);
    std::fs::copy(existing_audio_file, &new_audio_file).unwrap();
        
    let app_with_state = announcer::create_app(None, None).unwrap();

    let req = Request::new(
        Method::Delete,
        Url::parse("https://example.com/message/soft-bells.mp3").unwrap(),
    );
    let mut res: Response = app_with_state.app.respond(req).await.unwrap();

    let file_exists = std::fs::metadata(&new_audio_file).is_ok();

    if file_exists {
        std::fs::remove_file(new_audio_file).unwrap();
    }

    assert!(!file_exists);
}