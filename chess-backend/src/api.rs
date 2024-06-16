use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Player {
    Black,
    White,
}

#[derive(Serialize, Deserialize, Debug)]
enum PawnColor {
    Black,
    White,
}

#[derive(Serialize, Deserialize, Debug)]
enum PawnType {
    Pawn,
    Dame,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardState {
    id: String,
    content: (PawnColor, PawnType),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    player: Player,
    board_state: Vec<BoardState>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct GameState {
//     player: String,
//     board_state: HashMap<String, (String, String)>,
// }
// TODO something  not right with structs
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
