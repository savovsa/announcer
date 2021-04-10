use crate::messages::Message;
use rodio;
use serde::{Deserialize, Serialize};

pub const UNRECOGNIZED_FILE_FORMAT_ERROR_MESSAGE: &str =
    "Unrecognized file format, please upload MP3, WAV, Vorbis or FLAC.";

#[derive(Serialize, Deserialize)]
pub struct FileWithMeta {
    pub file: Vec<u8>,
    pub meta: Message,
}

pub mod endpoints {
    use super::*;
    use crate::{messages::save_config, Request};
    use async_std::{fs::OpenOptions, io};
    use std::path::Path;
    use tide;

    pub async fn upload(mut req: Request) -> tide::Result {
        let FileWithMeta { file, meta } = req.body_json().await?;

        let parsing_result = parse_audio_file(file).await;

        if parsing_result.is_err() {
            let mut res = tide::Response::new(400);
            res.set_body(UNRECOGNIZED_FILE_FORMAT_ERROR_MESSAGE);

            return Ok(res);
        }

        let bytes = parsing_result.unwrap();

        let name: String = req.param("name")?.parse()?;

        let file_path = {
            let config = &req.state().lock().unwrap();
            Path::new(&config.audio_folder_path).join(&name)
        };

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&file_path)
            .await?;

        io::copy(io::Cursor::new(bytes), file).await?;

        let config = &mut req.state().lock().unwrap();
        config.messages.insert(name, meta);
        save_config(config, None);

        let res = tide::Response::new(200);
        Ok(res)
    }
}

async fn parse_audio_file(bytes: Vec<u8>) -> Result<Vec<u8>, rodio::decoder::DecoderError> {
    let cursor = std::io::Cursor::new(bytes.clone());
    let rodio_result = rodio::decoder::Decoder::new(cursor);

    rodio_result.map(|_| bytes)
}
