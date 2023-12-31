use actix_web::{self, HttpServer, App};

mod handlers;
mod structs;

use handlers::get_handlers::{landing, list, get_assets, get_history};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(landing)
            .service(list)
            .service(get_assets)
            .service(get_history)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
