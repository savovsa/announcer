use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use announcer::{
    create_app,
    messages::{load_config, Config},
    AppWithState,
};
use notify::{
    event::ModifyKind, Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use rodio::{OutputStream, Sink};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sink = Arc::new(Mutex::new(sink));
    let cloned_sink = sink.clone();

    let AppWithState { app, state } = create_app(None, Some(cloned_sink)).unwrap();

    let mut watcher: RecommendedWatcher =
        RecommendedWatcher::new(move |result: Result<Event, Error>| {
            let event = result.unwrap();
            println!("hello");
            if event.kind == EventKind::Modify(ModifyKind::Any) {
                match load_config(&Config::get_path()) {
                    Ok(new_config) => {
                        state.lock().unwrap().update_config(new_config);
                    }
                    Err(error) => println!("Error reloading config: {:?}", error),
                }
            }
        })?;

    watcher.watch(&Path::new("announcer.json"), RecursiveMode::Recursive)?;
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
