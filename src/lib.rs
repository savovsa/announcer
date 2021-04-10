pub mod messages;
pub mod upload;

use messages::{endpoints::*, load_config, save_config, Config};
use notify::{
    event::ModifyKind, Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::sync::{Arc, Mutex};
use upload::endpoints::*;

pub fn create_app() -> tide::Result<tide::Server<Arc<Mutex<Config>>>> {
    let config_path = Config::get_path();

    let config = load_config(&config_path).unwrap_or_else(|_| {
        let default_config = Config::new();
        save_config(&default_config, None);

        default_config
    });
    let config = Arc::new(Mutex::new(config));
    let cloned_config = Arc::clone(&config);

    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |result: Result<Event, Error>| {
            let event = result.unwrap();

            if event.kind == EventKind::Modify(ModifyKind::Any) {
                match load_config(&Config::get_path()) {
                    Ok(new_config) => *cloned_config.lock().unwrap() = new_config,
                    Err(error) => println!("Error reloading config: {:?}", error),
                }
            }
        })?;

    watcher.watch(&config_path, RecursiveMode::Recursive)?;

    let mut app = tide::with_state(config);

    app.at("/messages").get(get_messages);
    app.at("/message/:name").get(get_message);
    app.at("/upload/:name").put(upload);

    Ok(app)
}

pub type Request = tide::Request<Arc<Mutex<Config>>>;
