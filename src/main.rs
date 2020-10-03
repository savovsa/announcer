mod messages;

use messages::{save_config, Config, Message};

fn main() {
    let config = Config {
        audio_folder_path: "sounds/",
        messages: [(
            "sound.mp3",
            Message {
                display_name: "Sound 1",
                volume: 60_f32,
            },
        )]
        .iter()
        .cloned()
        .collect(),
    };

    save_config(config);
}
