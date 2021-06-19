use announcer::{create_app, upload};
use announcer::{
    messages::{load_config, Config, Message},
    upload::FileWithMeta,
};
use async_std;
use k9::assert_equal;

const MP3_FILE_PATH: &str = "tests/soft-bells.mp3";

#[async_std::test]
async fn audio_file_gets_saved() -> surf::Result<()> {
    // remove the audio directory so that we test if we create it
    std::fs::remove_dir_all("sounds").unwrap_or(());

    let file_name = "soft-bells.mp3";
    let file_path = std::path::Path::new("sounds").join(file_name);
    let uri = "http://localhost:8080/upload/soft-bells.mp3";

    let app_with_state = create_app(None).unwrap();

    let file = std::fs::read(MP3_FILE_PATH).unwrap();
    let meta = Message {
        volume: 1.0,
        display_name: "soft-bells.mp3".to_string(),
    };
    let file_with_meta = FileWithMeta { file, meta };
    let body = surf::Body::from_json(&file_with_meta).unwrap();

    let res = surf::Client::with_http_client(app_with_state.app)
        .put(uri)
        .body(body)
        .await?;

    let file_exists = std::fs::metadata(file_path.clone()).is_ok();

    if file_exists {
        std::fs::remove_file(file_path).unwrap();
    }

    // TODO: Find a nicer way to test things like this
    // that prints why an HTTP error happened
    assert_eq!(res.status(), surf::StatusCode::Ok);
    assert!(file_exists);

    Ok(())
}

#[async_std::test]
async fn non_audio_file_doesnt_get_saved() {
    let file_name = "hello.wav";
    let file_path = std::path::Path::new("sounds").join(file_name);
    let uri = "http://localhost:8080/upload/hello.wav";

    let body = FileWithMeta {
        file: vec![],
        meta: Message {
            volume: 1.0,
            display_name: "".to_string(),
        },
    };

    let app_with_state = create_app(None).unwrap();
    let mut res = surf::Client::with_http_client(app_with_state.app)
        .put(uri)
        .body(surf::Body::from_json(&body).unwrap())
        .await
        .unwrap();

    let file_exists = std::fs::metadata(file_path.clone()).is_ok();

    if file_exists {
        std::fs::remove_file(file_path).unwrap();
    }

    // TODO: Find a nicer way to test things like this
    // that prints why an error happened
    assert_eq!(res.status(), surf::StatusCode::BadRequest);

    let body = res.body_string().await.unwrap();
    assert_eq!(body, upload::UNRECOGNIZED_FILE_FORMAT_ERROR_MESSAGE);
    assert!(!file_exists);
}

#[async_std::test]
async fn config_is_updated_after_successful_uploading() {
    let file_path = std::path::Path::new("sounds").join("soft-bells.mp3");
    let uri = "http://localhost:8080/upload/soft-bells.mp3";

    let app_with_state = create_app(None).unwrap();

    let file = std::fs::read(MP3_FILE_PATH).unwrap();

    let meta = Message {
        volume: 1.0,
        display_name: "soft-bells.mp3".to_string(),
    };

    let file_with_meta = FileWithMeta {
        file,
        meta: meta.clone(),
    };
    let body = surf::Body::from_json(&file_with_meta).unwrap();

    surf::Client::with_http_client(app_with_state.app)
        .put(uri)
        .body(body)
        .await
        .unwrap();

    let file_exists = std::fs::metadata(file_path.clone()).is_ok();

    if file_exists {
        std::fs::remove_file(file_path).unwrap();
    }

    let config = load_config(&Config::get_path()).unwrap();
    let message = config.messages.get("soft-bells.mp3").unwrap();

    assert_equal!(meta, *message);
}
