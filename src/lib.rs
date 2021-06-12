pub mod messages;
pub mod upload;

use rodio::{OutputStream, Sink};
use messages::{endpoints::*, load_config, save_config, Config};
use notify::{
    event::ModifyKind, Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::sync::{Arc, Mutex};
use upload::endpoints::*;

#[derive(Clone)]
pub struct AppState {
    pub sink: Arc<Mutex<Sink>>,
    pub config: Arc<Mutex<Config>>
}

impl AppState {
    fn update_config(&mut self, new_config: Config) { 
        let mut config = self.config.lock().unwrap();
        *config = new_config;
    }
}

pub type State = Arc<Mutex<AppState>>;
pub type Request = tide::Request<State>;

pub fn create_app(config: Option<Config>) -> tide::Result<tide::Server<State>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let config_path = Config::get_path();
    let config = config.unwrap_or_else(|| {
        load_config(&config_path).unwrap_or_else(|_| {
            let default_config = Config::new();
            save_config(&default_config, None);

            default_config
        })
    });

    let app_state = AppState {
        sink: Arc::new(Mutex::new(sink)),
        config: Arc::new(Mutex::new(config))
    };

    let state: State = Arc::new(Mutex::new(app_state));
    let cloned_state = Arc::clone(&state);

    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |result: Result<Event, Error>| {
            let event = result.unwrap();

            if event.kind == EventKind::Modify(ModifyKind::Any) {
                match load_config(&Config::get_path()) {
                    Ok(new_config) => {
                        cloned_state.lock().unwrap().update_config(new_config);
                        
                    },
                    Err(error) => println!("Error reloading config: {:?}", error),
                }
            }
        })?;

    watcher.watch(&config_path, RecursiveMode::Recursive)?;

    let mut app = tide::with_state(state);

    app.at("/messages").get(get_messages);
    app.at("/message/:name").get(get_message);
    app.at("/upload/:name").put(upload);
    app.at("/play/:name").get(play_message);

    Ok(app)
}
