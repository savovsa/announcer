use std::collections::HashMap;

mod messages;

fn main() {
    let config = messages::Config {
        audio_folder_path: "sounds/",
        messages: HashMap::new(),
    };

    messages::save_config(config);
}
