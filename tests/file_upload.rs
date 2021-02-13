use announcer::create_app;
use announcer::messages::load_config;
use async_std;
use surf;

#[async_std::test]
async fn file_gets_saved() -> surf::Result<()> {
    let file_name = "hello.wav";
    let file_path = std::path::Path::new("sounds").join(file_name);
    let uri = "http://localhost:8080/upload/hello.wav";
    let data = load_config("tests/messages_test_config.json").unwrap();

    let app = create_app().unwrap();
    let res = surf::Client::with_http_client(app)
        .put(uri)
        .body(surf::Body::from_json(&data)?)
        .await?;

    let file_exists = std::fs::metadata(file_path.clone()).is_ok();

    if file_exists {
        std::fs::remove_file(file_path).unwrap();
    }

    assert_eq!(res.status(), surf::StatusCode::Ok);
    assert!(file_exists);

    Ok(())
}
