use announcer::messages::{load_config, save_config, Config, Message};
use tide::{http, Body, Response};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config = load_config("config.json").unwrap();

    let mut app = tide::with_state(config);

    app.at("/messages").get(get_messages);
    app.at("/message/:name").get(get_message);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

type Request = tide::Request<Config>;

async fn get_messages(req: Request) -> tide::Result {
    let mut res = Response::new(200);
    let body = Body::from_json(&req.state().messages)?;
    res.set_body(body);
    Ok(res)
}

async fn get_message(req: Request) -> tide::Result {
    let mut res = Response::new(200);

    let name: String = req.param("name")?.parse()?;
    let value = req.state().messages.get(&name);

    let body = Body::from_json(&value)?;
    res.set_body(body);
    Ok(res)
}
