use announcer::create_app;
use announcer::messages::load_config;
use async_std;
use surf::{self, Body};

#[async_std::test]
async fn audio_file_gets_saved() -> surf::Result<()> {
    let file_name = "soft-bells.mp3";
    let file_path = std::path::Path::new("sounds").join(file_name);
    let uri = "http://localhost:8080/upload/soft-bells.mp3";

    let app = create_app().unwrap();

    let body = Body::from_file("tests/soft-bells.mp3").await?;
    let mut res = surf::Client::with_http_client(app)
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
    assert_eq!(body, "Unrecognized file format");
    assert!(!file_exists);
}
