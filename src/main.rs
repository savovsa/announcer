use std::path::Path;

use announcer::{AppWithState, create_app, messages::{Config, load_config}};
use notify::{Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let AppWithState {app, state} = create_app(None).unwrap();

    let mut watcher: RecommendedWatcher =
    RecommendedWatcher::new(move |result: Result<Event, Error>| {
        let event = result.unwrap();
        println!("hello");
        if event.kind == EventKind::Modify(ModifyKind::Any) {
            match load_config(&Config::get_path()) {
                Ok(new_config) => {
                    state.lock().unwrap().update_config(new_config);
                    
                },
                Err(error) => println!("Error reloading config: {:?}", error),
            }
        }
    })?;

    watcher.watch(&Path::new("announcer.json"), RecursiveMode::Recursive)?;
    app.listen("127.0.0.1:8080").await?;
    
    Ok(())
}
