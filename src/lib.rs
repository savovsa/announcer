pub mod messages;
pub mod upload;

use messages::{endpoints::*, load_config, save_config, Config};
use notify::{
    event::ModifyKind, Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::sync::{Arc, Mutex};
use upload::endpoints::*;

pub type State = Arc<Mutex<Config>>;
pub type Request = tide::Request<State>;

pub fn create_app(config: Option<Config>) -> tide::Result<tide::Server<State>> {
    let config_path = Config::get_path();
    let config = config.unwrap_or_else(|| {
        load_config(&config_path).unwrap_or_else(|_| {
            let default_config = Config::new();
            save_config(&default_config, None);

            default_config
        })
    });

    let state: State = Arc::new(Mutex::new(config));
    let cloned_state = Arc::clone(&state);

    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |result: Result<Event, Error>| {
            let event = result.unwrap();

            if event.kind == EventKind::Modify(ModifyKind::Any) {
                match load_config(&Config::get_path()) {
                    Ok(new_config) => *cloned_state.lock().unwrap() = new_config,
                    Err(error) => println!("Error reloading config: {:?}", error),
                }
            }
        })?;

    watcher.watch(&config_path, RecursiveMode::Recursive)?;

    let mut app = tide::with_state(state);

    app.at("/messages").get(get_messages);
    app.at("/message/:name").get(get_message);
    app.at("/upload/:name").put(upload);

    Ok(app)
}
