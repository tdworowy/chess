mod api;
mod game;
use api::{get_example, healthcheck, make_move_api, make_random_move_api};

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(make_move_api)
            .service(make_random_move_api)
            .service(healthcheck)
            .service(get_example)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
