use crate::Request;
use rodio;

pub mod endpoints {
    use super::*;
    use crate::Request;
    use async_std::{fs::OpenOptions, io};
    use std::path::Path;
    use tide;

    pub async fn upload(mut req: Request) -> tide::Result {
        // Using a &mut because reading the request body as bytes requires it
        let bytes = parse_audio_file(&mut req).await?;
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
