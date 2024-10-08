use actix_web::{get, options, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use game::{get_start_board, make_random_move, GameState, Player};

use crate::game;

#[post("/make_move")]
pub async fn make_move_api(game_state: web::Json<GameState>) -> impl Responder {
    let game_state = game_state.into_inner();
    match serde_json::to_string(&game_state) {
        Ok(game_state_json) => HttpResponse::Ok().json(game_state_json),
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError()
                .body(format!("make_move Error: {:?}", error.to_string()))
        }
    }
}

#[post("/make_radnom_move")]
pub async fn make_random_move_api(game_state: web::Json<GameState>) -> impl Responder {
    let game_state = game_state.into_inner();
    let new_game_state = make_random_move(game_state);
    match serde_json::to_string(&new_game_state) {
        Ok(new_game_state_json) => HttpResponse::Ok()
            .append_header(("Access-Control-Allow-Origin", "*"))
            .json(new_game_state_json),
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError()
                .body(format!("make_move Error: {:?}", error.to_string()))
        }
    }
}

#[options("/make_radnom_move")]
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
    match serde_json::to_string(&game_state) {
        Ok(game_state_json) => HttpResponse::Ok().json(game_state_json),
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError()
                .body(format!("get_example Error: {:?}", error.to_string()))
        }
    }
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
    match serde_json::to_string(&healthcheck_struct) {
        Ok(healthcheck_json) => HttpResponse::Ok()
            .append_header(("Access-Control-Allow-Origin", "*"))
            .json(healthcheck_json),
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError()
                .body(format!("Healthcheck Error: {:?}", error.to_string()))
        }
    }
}

#[options("/healthcheck")]
pub async fn healthcheck_options() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Allow", "OPTIONS, GET"))
        .append_header(("Access-Control-Allow-Methods", "GET, OPTIONS"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .finish()
}
