mod api;
use api::{get_example, healthcheck, make_move};

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(make_move)
            .service(healthcheck)
            .service(get_example)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
