use crate::Request;
use rodio;

pub const UNRECOGNIZED_FILE_FORMAT_ERROR_MESSAGE: &str =
    "Unrecognized file format, please upload MP3, WAV, Vorbis or FLAC.";

pub mod endpoints {
    use super::*;
    use crate::Request;
    use async_std::{fs::OpenOptions, io};
    use std::path::Path;
    use tide;

    pub async fn upload(mut req: Request) -> tide::Result {
        // Using a &mut because reading the request body as bytes requires it
        let parsing_result = parse_audio_file(&mut req).await;

        if parsing_result.is_err() {
            let mut res = tide::Response::new(400);
            res.set_body(UNRECOGNIZED_FILE_FORMAT_ERROR_MESSAGE);

            return Ok(res);
        }

        let bytes = parsing_result.unwrap();

        let name: String = req.param("name")?.parse()?;

        let file_path = {
            let config = &req.state().lock().unwrap();
            Path::new(&config.audio_folder_path).join(name)
        };

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&file_path)
            .await?;

        io::copy(io::Cursor::new(bytes), file).await?;

        let res = tide::Response::new(200);
        Ok(res)
    }
}

async fn parse_audio_file(req: &mut Request) -> Result<Vec<u8>, rodio::decoder::DecoderError> {
    let bytes = req.body_bytes().await.unwrap();
    let cursor = std::io::Cursor::new(bytes.clone());
    let rodio_result = rodio::decoder::Decoder::new(cursor);

    rodio_result.map(|_| bytes)
}
