use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use game::{get_start_board, GameState, Player};

use crate::game;

#[post("/make_move")]
pub async fn make_move(game_state: web::Json<GameState>) -> impl Responder {
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
        Ok(healthcheck_json) => HttpResponse::Ok().json(healthcheck_json),
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError()
                .body(format!("Healthcheck Error: {:?}", error.to_string()))
        }
    }
}
