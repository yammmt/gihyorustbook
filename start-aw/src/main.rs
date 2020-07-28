use actix_web::{web, App, HttpServer, Responder};

use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    name: Option<String>,
}

async fn hello(path: web::Path<HelloPath>) -> impl Responder {
    let to = path.name.as_ref().map(|s| s.as_ref()).unwrap_or("World");
    format!("Hello {}!", to)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/{name}", web::get().to(hello))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run()
    .await
}
