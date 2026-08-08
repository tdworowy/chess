use actix_web::{get, options, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use game::{get_start_board, make_random_move, GameState, Player};

use crate::game;

#[post("/make_move")]
pub async fn make_move_api(game_state: web::Json<GameState>) -> impl Responder {
    let game_state = game_state.into_inner();
    HttpResponse::Ok().json(game_state)
}

#[post("/make_random_move")]
pub async fn make_random_move_api(game_state: web::Json<GameState>) -> impl Responder {
    let game_state = game_state.into_inner();
    let new_game_state = make_random_move(game_state);
    match new_game_state {
        Some(state) => HttpResponse::Ok()
            .append_header(("Access-Control-Allow-Origin", "*"))
            .json(state),
        None => HttpResponse::BadRequest()
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body("No available moves"),
    }
}

#[options("/make_random_move")]
pub async fn make_random_move_options_api() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Allow", "OPTIONS, POST"))
        .append_header(("Access-Control-Allow-Methods", "POST, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .finish()
}

#[get("/get_example")]
pub async fn get_example() -> impl Responder {
    let game_state = GameState {
        player: Player::Black,
        board_state: get_start_board(),
    };
    HttpResponse::Ok().json(game_state)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Healthcheck {
    message: String,
}
#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    let healthcheck_struct = Healthcheck {
        message: "OK".to_owned(),
    };
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .json(healthcheck_struct)
}

#[options("/healthcheck")]
pub async fn healthcheck_options() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Allow", "OPTIONS, GET"))
        .append_header(("Access-Control-Allow-Methods", "GET, OPTIONS"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .finish()
}
