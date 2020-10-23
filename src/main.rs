use announcer::messages::{save_config, Config, Message};

fn main() {
    let config = Config {
        audio_folder_path: "sounds/".to_string(),
        messages: [(
            "sound.mp3".to_string(),
            Message {
                display_name: "Sound 1".to_string(),
                volume: 60_f32,
            },
        )]
        .iter()
        .cloned()
        .collect(),
    };

    save_config(config);
}
