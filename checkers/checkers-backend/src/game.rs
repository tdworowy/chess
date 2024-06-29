use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::format};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Player {
    Black,
    White,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PawnColor {
    Empty,
    Black,
    White,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PawnType {
    Empty,
    Pawn,
    Dame,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldState {
    pub pawn_color: PawnColor,
    pub pawn_type: PawnType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub player: Player,
    pub board_state: HashMap<String, FieldState>,
}

pub struct AvaiableActions {
    pawns_can_move: Vec<String>,
    pawns_can_beat: Vec<String>,
    dames_can_move: Vec<String>,
    dames_can_beat: Vec<String>,
}

pub fn get_start_board() -> HashMap<String, FieldState> {
    let mut start_board: HashMap<String, FieldState> = HashMap::new();
    for i in 1..9 {
        for j in 1..9 {
            if (i == 1 || i == 3) && j % 2 == 0 {
                start_board.insert(
                    format!("{}_{}", i, j),
                    FieldState {
                        pawn_color: PawnColor::Black,
                        pawn_type: PawnType::Pawn,
                    },
                );
                continue;
            }
            if i == 2 && j % 2 != 0 {
                start_board.insert(
                    format!("{}_{}", i, j),
                    FieldState {
                        pawn_color: PawnColor::Black,
                        pawn_type: PawnType::Pawn,
                    },
                );
                continue;
            }
            if (i == 6 || i == 8) && j % 2 != 0 {
                start_board.insert(
                    format!("{}_{}", i, j),
                    FieldState {
                        pawn_color: PawnColor::White,
                        pawn_type: PawnType::Pawn,
                    },
                );
                continue;
            }
            if i == 7 && j % 2 == 0 {
                start_board.insert(
                    format!("{}_{}", i, j),
                    FieldState {
                        pawn_color: PawnColor::White,
                        pawn_type: PawnType::Pawn,
                    },
                );
                continue;
            }

            start_board.insert(
                format!("{}_{}", i, j),
                FieldState {
                    pawn_color: PawnColor::Empty,
                    pawn_type: PawnType::Empty,
                },
            );
        }
    }

    start_board
}

fn get_avaiable_actions(game_state: GameState) -> AvaiableActions {
    let mut pawns_can_move: Vec<String> = Vec::new();
    let mut pawns_can_beat: Vec<String> = Vec::new();
    let mut dames_can_move: Vec<String> = Vec::new();
    let mut dames_can_beat: Vec<String> = Vec::new();

    let pawn_color = match game_state.player {
        Player::Black => PawnColor::Black,
        Player::White => PawnColor::White,
    };

    let move_funcion = match game_state.player {
        Player::Black => can_black_pawn_move,
        Player::White => can_white_pawn_move,
    };

    let beat_function = match game_state.player {
        Player::Black => can_black_pawn_beat,
        Player::White => can_white_pawn_beat,
    };

    game_state
        .clone()
        .board_state
        .into_iter()
        .for_each(|state| match state.1 {
            FieldState {
                pawn_color,
                pawn_type: PawnType::Pawn,
            } => {
                if (move_funcion(game_state.clone(), state.0.clone())) {
                    pawns_can_move.push(state.0.clone())
                }
                if (beat_function(game_state.clone(), state.0.clone())) {
                    pawns_can_beat.push(state.0.clone())
                }
            }
            FieldState {
                pawn_color,
                pawn_type: PawnType::Dame,
            } => {
                if can_dame_move(game_state.clone(), state.0.clone()) {
                    dames_can_move.push(state.0.clone())
                }
                if can_dame_beat(game_state.clone(), state.0.clone()) {
                    dames_can_beat.push(state.0.clone())
                }
            }
            _ => {}
        });
    AvaiableActions {
        pawns_can_move,
        pawns_can_beat,
        dames_can_move,
        dames_can_beat,
    }
}

fn can_black_pawn_move(game_state: GameState, position: String) -> bool {
    let _position: Vec<&str> = position.split("_").collect();
    let mut result: bool = false;
    let x: u32 = _position[0].parse().expect("not a number");
    let y: u32 = _position[1].parse().expect("not a number");
    if x <= 8 && y - 1 > 0 {
        match game_state.board_state.get(&format!("{}_{}", x + 1, y - 1)) {
            Some(state) => {
                match state.pawn_type {
                    PawnType::Empty => result = true,
                    PawnType::Pawn => {}
                    PawnType::Dame => {}
                };
            }
            None => {}
        };
    };
    if x <= 8 && y + 1 <= 8 {
        match game_state.board_state.get(&format!("{}_{}", x + 1, y + 1)) {
            Some(state) => {
                match state.pawn_type {
                    PawnType::Empty => result = true,
                    PawnType::Pawn => {}
                    PawnType::Dame => {}
                };
            }
            None => {}
        }
    };

    result
}

fn can_white_pawn_move(game_state: GameState, position: String) -> bool {
    let _position: Vec<&str> = position.split("_").collect();
    let mut result: bool = false;
    let x: u32 = _position[0].parse().expect("not a number");
    let y: u32 = _position[1].parse().expect("not a number");
    if x >= 1 && y - 1 > 0 {
        match game_state.board_state.get(&format!("{}_{}", x - 1, y - 1)) {
            Some(state) => {
                match state.pawn_type {
                    PawnType::Empty => result = true,
                    PawnType::Pawn => {}
                    PawnType::Dame => {}
                };
            }
            None => {}
        };
    };
    if x >= 1 && y + 1 <= 8 {
        match game_state.board_state.get(&format!("{}_{}", x - 1, y + 1)) {
            Some(state) => {
                match state.pawn_type {
                    PawnType::Empty => result = true,
                    PawnType::Pawn => {}
                    PawnType::Dame => {}
                };
            }
            None => {}
        }
    };

    result
}

fn can_black_pawn_beat(game_state: GameState, position: String) -> bool {
    let _position: Vec<&str> = position.split("_").collect();
    let mut result: bool = false;

    let x: u32 = _position[0].parse().expect("not a number");
    let y: u32 = _position[1].parse().expect("not a number");
    if x <= 8 && y - 1 > 0 {
        match game_state.board_state.get(&format!("{}_{}", x + 1, y - 1)) {
            Some(state) => {
                match state {
                    FieldState {
                        pawn_type: PawnType::Pawn,
                        pawn_color: PawnColor::White,
                    } => {
                        result =
                            can_black_pawn_move(game_state.clone(), format!("{}_{}", x + 1, y - 1))
                    }
                    _ => {}
                };
            }
            None => {}
        };
    };
    if x <= 8 && y + 1 <= 8 {
        match game_state.board_state.get(&format!("{}_{}", x + 1, y + 1)) {
            Some(state) => {
                match state {
                    FieldState {
                        pawn_type: PawnType::Pawn,
                        pawn_color: PawnColor::White,
                    } => {
                        result =
                            can_black_pawn_move(game_state.clone(), format!("{}_{}", x + 1, y + 1))
                    }
                    _ => {}
                };
            }
            None => {}
        }
    };
    result
}

fn can_white_pawn_beat(game_state: GameState, position: String) -> bool {
    let _position: Vec<&str> = position.split("_").collect();
    let mut result: bool = false;

    let x: u32 = _position[0].parse().expect("not a number");
    let y: u32 = _position[1].parse().expect("not a number");
    if x >= 1 && y - 1 > 0 {
        match game_state.board_state.get(&format!("{}_{}", x - 1, y - 1)) {
            Some(state) => {
                match state {
                    FieldState {
                        pawn_type: PawnType::Pawn,
                        pawn_color: PawnColor::Black,
                    } => {
                        result =
                            can_white_pawn_move(game_state.clone(), format!("{}_{}", x - 1, y - 1))
                    }
                    _ => {}
                };
            }
            None => {}
        };
    };
    if x >= 1 && y + 1 <= 8 {
        match game_state.board_state.get(&format!("{}_{}", x - 1, y + 1)) {
            Some(state) => {
                match state {
                    FieldState {
                        pawn_type: PawnType::Pawn,
                        pawn_color: PawnColor::Black,
                    } => {
                        result =
                            can_white_pawn_move(game_state.clone(), format!("{}_{}", x - 1, y + 1))
                    }
                    _ => {}
                };
            }
            None => {}
        }
    };
    result
}

fn can_dame_move(game_state: GameState, position: String) -> bool {
    true
}
fn can_dame_beat(game_state: GameState, position: String) -> bool {
    true
}

#[test]
fn test_can_black_pawn_move() {
    let game_state = GameState {
        player: Player::Black,
        board_state: get_start_board(),
    };
    assert!(can_black_pawn_move(game_state.clone(), "3_2".to_string()));
    assert!(can_black_pawn_move(game_state.clone(), "3_4".to_string()));
    assert!(can_black_pawn_move(game_state.clone(), "3_8".to_string()));

    assert!(can_black_pawn_move(game_state.clone(), "2_1".to_string()) == false);
    assert!(can_black_pawn_move(game_state.clone(), "2_3".to_string()) == false);
    assert!(can_black_pawn_move(game_state.clone(), "2_7".to_string()) == false);
}

#[test]
fn test_can_white_pawn_move() {
    let game_state = GameState {
        player: Player::White,
        board_state: get_start_board(),
    };
    assert!(can_white_pawn_move(game_state.clone(), "6_1".to_string()));
    assert!(can_white_pawn_move(game_state.clone(), "6_3".to_string()));
    assert!(can_white_pawn_move(game_state.clone(), "6_7".to_string()));

    assert!(can_white_pawn_move(game_state.clone(), "7_2".to_string()) == false);
    assert!(can_white_pawn_move(game_state.clone(), "7_4".to_string()) == false);
    assert!(can_white_pawn_move(game_state.clone(), "7_8".to_string()) == false);
}

//TODO test can_black_pawn_beat and can_white_pawn_beat
