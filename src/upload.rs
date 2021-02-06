pub mod endpoints {
    use crate::{messages::Config, Request};
    use async_std::{fs::OpenOptions, io};
    use std::path::Path;
    use tide;

    pub async fn upload(mut req: Request) -> tide::Result {
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

        let body = req.body_bytes().await.unwrap();

        io::copy(io::Cursor::new(body), file).await?;

        let res = tide::Response::new(200);
        Ok(res)
    }
}
