use actix_web::{get, web, App, HttpServer, Responder};

pub mod messages;

async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }

fn main() {
    let config = messages::Config {
        message: "Message 1".to_string(),
    };

    messages::save_config(config);
}
