use announcer::messages::{load_config, save_config, Config, Message};
use notify::{
    event::ModifyKind, Error, Event, EventFn, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::sync::{Arc, Mutex};
use tide::{http, Body, Response};

const CONFIG_PATH: &str = "config.json";

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config = load_config(CONFIG_PATH).unwrap();
    let config = Arc::new(Mutex::new(config));

    let moved_config = Arc::clone(&config);

    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |result: Result<Event, Error>| {
            let event = result.unwrap();

            if event.kind == EventKind::Modify(ModifyKind::Any) {
                println!("Event {:?}", event.kind);
                let mut config_guard = moved_config.lock().unwrap();
                *config_guard = load_config(CONFIG_PATH).unwrap();
            }
        })?;

    watcher
        .configure(notify::Config::OngoingEvents(Some(
            std::time::Duration::from_secs(1),
        )))
        .unwrap();

    watcher.watch(CONFIG_PATH, RecursiveMode::Recursive)?;

    let mut app = tide::with_state(config);

    app.at("/messages").get(get_messages);
    app.at("/message/:name").get(get_message);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

type Request = tide::Request<Arc<Mutex<Config>>>;

async fn get_messages(req: Request) -> tide::Result {
    let mut res = Response::new(200);
    let config = &req.state().lock().unwrap();
    let body = Body::from_json(&config.messages)?;
    res.set_body(body);
    Ok(res)
}

async fn get_message(req: Request) -> tide::Result {
    let mut res = Response::new(200);

    let name: String = req.param("name")?.parse()?;
    let config = &req.state().lock().unwrap();
    let value = config.messages.get(&name);

    let body = Body::from_json(&value)?;
    res.set_body(body);
    Ok(res)
}
