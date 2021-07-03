pub mod messages;
pub mod upload;

use messages::{endpoints::*, load_config, save_config, Config};
use rodio::Sink;
use std::sync::{Arc, Mutex};
use upload::endpoints::*;

#[derive(Clone)]
pub struct AppState {
    pub sink: Arc<Mutex<Sink>>,
    pub config: Arc<Mutex<Config>>,
}

impl AppState {
    pub fn update_config(&mut self, new_config: Config) {
        let mut config = self.config.lock().unwrap();
        *config = new_config;
    }
}

pub type State = Arc<Mutex<AppState>>;
pub type Request = tide::Request<State>;
pub type App = tide::Server<State>;
pub struct AppWithState {
    pub app: App,
    pub state: State,
}

pub fn create_app(
    config: Option<Config>,
    sink: Option<Arc<Mutex<Sink>>>,
) -> tide::Result<AppWithState> {
    let config_path = Config::get_path();
    let config = config.unwrap_or_else(|| {
        load_config(&config_path).unwrap_or_else(|_| {
            let default_config = Config::new();
            save_config(&default_config, None);

            default_config
        })
    });

    let app_state = AppState {
        sink: sink.unwrap_or(Arc::new(Mutex::new(Sink::new_idle().0))),
        config: Arc::new(Mutex::new(config)),
    };

    let state: State = Arc::new(Mutex::new(app_state));
    let cloned_state = Arc::clone(&state);

    let mut app = tide::with_state(state);

    app.at("/messages").get(get_messages);
    app.at("/message/:name").get(get_message);
    app.at("/message/:name").delete(delete_message);

    app.at("/upload/:name").put(upload);
    app.at("/play/:name").get(play_message);

    Ok(AppWithState {
        app,
        state: cloned_state,
    })
}
