use announcer::{create_app, upload};
use announcer::{
    messages::{load_config, Message},
    upload::FileWithMeta,
};
use async_std;
use k9::assert_equal;
use surf::{self, Body};

const MP3_FILE_PATH: &str = "tests/soft-bells.mp3";

#[async_std::test]
async fn audio_file_gets_saved() -> surf::Result<()> {
    let file_name = "soft-bells.mp3";
    let file_path = std::path::Path::new("sounds").join(file_name);
    let uri = "http://localhost:8080/upload/soft-bells.mp3";

    let app = create_app().unwrap();

    let body = Body::from_file(MP3_FILE_PATH).await?;
    let res = surf::Client::with_http_client(app)
        .put(uri)
        .body(body)
        .await?;

    let file_exists = std::fs::metadata(file_path.clone()).is_ok();

    if file_exists {
        std::fs::remove_file(file_path).unwrap();
    }

    // TODO: Find a nicer way to test things like this
    // that prints why an error happened
    assert_eq!(res.status(), surf::StatusCode::Ok);
    assert!(file_exists);

    Ok(())
}

#[async_std::test]
async fn non_audio_file_doesnt_get_saved() {
    let file_name = "hello.wav";
    let file_path = std::path::Path::new("sounds").join(file_name);
    let uri = "http://localhost:8080/upload/hello.wav";
    let data = load_config("tests/messages_test_config.json").unwrap();

    let app = create_app().unwrap();
    let mut res = surf::Client::with_http_client(app)
        .put(uri)
        .body(surf::Body::from_json(&data).unwrap())
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

    let app = create_app().unwrap();

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

    surf::Client::with_http_client(app)
        .put(uri)
        .body(body)
        .await
        .unwrap();

    let file_exists = std::fs::metadata(file_path.clone()).is_ok();

    if file_exists {
        std::fs::remove_file(file_path).unwrap();
    }

    let config = load_config("config.json").unwrap();
    let message = config.messages.get("soft-bells.mp3").unwrap();

    assert_equal!(meta, *message);
}
