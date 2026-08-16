pub mod ai;
pub mod api;
pub mod game;

use api::{
    get_example, healthcheck, healthcheck_options, make_ai_move_api, make_ai_move_options_api,
    make_random_move_api, make_random_move_options_api,
};

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(make_ai_move_api)
            .service(make_ai_move_options_api)
            .service(make_random_move_api)
            .service(make_random_move_options_api)
            .service(healthcheck)
            .service(healthcheck_options)
            .service(get_example)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
