use announcer::create_app;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let app = create_app(None).unwrap();
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
