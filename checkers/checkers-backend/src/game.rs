use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::collections::{hash_map::Entry, HashMap};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Player {
    Black,
    White,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PawnColor {
    Empty,
    Black,
    White,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PawnType {
    Empty,
    Pawn,
    Dame,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FieldState {
    pub pawn_color: PawnColor,
    pub pawn_type: PawnType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub player: Player,
    pub board_state: HashMap<String, FieldState>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvaiableActions {
    pawns_can_move: HashMap<String, Vec<String>>,
    pawns_can_beat: HashMap<String, Vec<(String, String)>>,
    dames_can_move: HashMap<String, Vec<String>>,
    dames_can_beat: HashMap<String, Vec<(String, String)>>,
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
    let mut pawns_can_move: HashMap<String, Vec<String>> = HashMap::new();
    let mut pawns_can_beat: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut dames_can_move: HashMap<String, Vec<String>> = HashMap::new();
    let mut dames_can_beat: HashMap<String, Vec<(String, String)>> = HashMap::new();

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
                let can_move = move_funcion(game_state.clone(), &state.0);
                if can_move.0 {
                    match pawns_can_move.entry(state.0.clone()) {
                        Entry::Vacant(e) => {
                            e.insert(can_move.1.clone());
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().extend(can_move.1.clone());
                        }
                    }
                }
                let can_beat = beat_function(game_state.clone(), &state.0);
                if can_beat.0 {
                    match pawns_can_beat.entry(state.0) {
                        Entry::Vacant(e) => {
                            e.insert(can_beat.1.clone());
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().extend(can_beat.1);
                        }
                    }
                }
            }
            FieldState {
                pawn_color,
                pawn_type: PawnType::Dame,
            } => {
                let can_move = can_dame_move(game_state.clone(), &state.0);
                if can_move.0 {
                    match dames_can_move.entry(state.0.clone()) {
                        Entry::Vacant(e) => {
                            e.insert(can_move.1.clone());
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().extend(can_move.1.clone());
                        }
                    }
                }
                let can_beat = can_dame_beat(game_state.clone(), &state.0);
                if can_beat.0 {
                    match dames_can_beat.entry(state.0) {
                        Entry::Vacant(e) => {
                            e.insert(can_beat.1);
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().extend(can_beat.1);
                        }
                    }
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
fn is_position_free(board_state: &HashMap<String, FieldState>, position: &String) -> bool {
    match board_state.get(position) {
        Some(state) => match state.pawn_type {
            PawnType::Empty => true,
            PawnType::Pawn => false,
            PawnType::Dame => false,
        },
        None => false,
    }
}

fn can_black_pawn_move(game_state: GameState, position: &String) -> (bool, Vec<String>) {
    let _position: Vec<&str> = position.split("_").collect();
    let mut next_positions: Vec<String> = Vec::new();

    let x: u32 = _position[0].parse().expect("not a number");
    let y: u32 = _position[1].parse().expect("not a number");
    if x <= 8 && y - 1 > 0 {
        let next_position = format!("{}_{}", x + 1, y - 1);
        if is_position_free(&game_state.board_state, &next_position) {
            next_positions.push(next_position);
        }
    }

    if x <= 8 && y + 1 <= 8 {
        let next_position = format!("{}_{}", x + 1, y + 1);
        if is_position_free(&game_state.board_state, &next_position) {
            next_positions.push(next_position);
        }
    };
    if next_positions.len() > 0 {
        (true, next_positions)
    } else {
        (false, vec!["".to_string()])
    }
}

fn can_white_pawn_move(game_state: GameState, position: &String) -> (bool, Vec<String>) {
    let _position: Vec<&str> = position.split("_").collect();
    let mut next_positions: Vec<String> = Vec::new();
    let x: u32 = _position[0].parse().expect("not a number");
    let y: u32 = _position[1].parse().expect("not a number");
    if x >= 1 && y - 1 > 0 {
        let next_position = format!("{}_{}", x - 1, y - 1);
        if is_position_free(&game_state.board_state, &next_position) {
            next_positions.push(next_position);
        }
    };
    if x >= 1 && y + 1 <= 8 {
        let next_position = format!("{}_{}", x - 1, y + 1);
        if is_position_free(&game_state.board_state, &next_position) {
            next_positions.push(next_position);
        }
    };

    if next_positions.len() > 0 {
        (true, next_positions)
    } else {
        (false, vec!["".to_string()])
    }
}

fn can_black_pawn_beat(game_state: GameState, position: &String) -> (bool, Vec<(String, String)>) {
    let _position: Vec<&str> = position.split("_").collect();
    let mut enemy_and_next_positions: Vec<(String, String)> = Vec::new();

    let x: u32 = _position[0].parse().expect("not a number");
    let y: u32 = _position[1].parse().expect("not a number");
    if x <= 8 && y - 1 > 0 {
        let enemy_position = format!("{}_{}", x + 1, y - 1);
        match game_state.board_state.get(&enemy_position) {
            Some(state) => match state {
                FieldState {
                    pawn_type: PawnType::Pawn,
                    pawn_color: PawnColor::White,
                } => {
                    let new_position = format!("{}_{}", x + 2, y - 2);
                    if is_position_free(&game_state.board_state, &new_position) {
                        enemy_and_next_positions.push((enemy_position, new_position));
                    }
                }
                _ => {}
            },
            None => {}
        }
    };
    if x <= 8 && y + 1 <= 8 {
        let enemy_position = format!("{}_{}", x + 1, y + 1);
        match game_state.board_state.get(&enemy_position) {
            Some(state) => {
                match state {
                    FieldState {
                        pawn_type: PawnType::Pawn,
                        pawn_color: PawnColor::White,
                    } => {
                        let new_position = format!("{}_{}", x + 2, y + 2);
                        if is_position_free(&game_state.board_state, &new_position) {
                            enemy_and_next_positions.push((enemy_position, new_position));
                        }
                    }
                    _ => {}
                };
            }
            None => {}
        }
    };
    if enemy_and_next_positions.len() > 0 {
        (true, enemy_and_next_positions)
    } else {
        (false, vec![("".to_string(), "".to_string())])
    }
}

fn can_white_pawn_beat(game_state: GameState, position: &String) -> (bool, Vec<(String, String)>) {
    let _position: Vec<&str> = position.split("_").collect();
    let mut enemy_and_next_positions: Vec<(String, String)> = Vec::new();

    let x: u32 = _position[0].parse().expect("not a number");
    let y: u32 = _position[1].parse().expect("not a number");
    if x >= 1 && y - 1 > 0 {
        let enemy_position = format!("{}_{}", x - 1, y - 1);
        match game_state.board_state.get(&enemy_position) {
            Some(state) => {
                match state {
                    FieldState {
                        pawn_type: PawnType::Pawn,
                        pawn_color: PawnColor::Black,
                    } => {
                        let new_position = format!("{}_{}", x - 2, y - 2);
                        if is_position_free(&game_state.board_state, &new_position) {
                            enemy_and_next_positions.push((enemy_position, new_position));
                        }
                    }
                    _ => {}
                };
            }
            None => {}
        };
    };
    if x >= 1 && y + 1 <= 8 {
        let enemy_position = format!("{}_{}", x - 1, y + 1);
        match game_state.board_state.get(&enemy_position) {
            Some(state) => {
                match state {
                    FieldState {
                        pawn_type: PawnType::Pawn,
                        pawn_color: PawnColor::Black,
                    } => {
                        let new_position = format!("{}_{}", x - 2, y + 2);
                        if is_position_free(&game_state.board_state, &new_position) {
                            enemy_and_next_positions.push((enemy_position, new_position));
                        }
                    }
                    _ => {}
                };
            }
            None => {}
        }
    };
    if enemy_and_next_positions.len() > 0 {
        (true, enemy_and_next_positions)
    } else {
        (false, vec![("".to_string(), "".to_string())])
    }
}

fn can_dame_move(game_state: GameState, position: &String) -> (bool, Vec<String>) {
    (false, vec!["".to_string()])
}
fn can_dame_beat(game_state: GameState, position: &String) -> (bool, Vec<(String, String)>) {
    (false, vec![("".to_string(), "".to_string())])
}

#[test]
fn test_can_black_pawn_move() {
    let game_state = GameState {
        player: Player::Black,
        board_state: get_start_board(),
    };
    assert_eq!(
        can_black_pawn_move(game_state.clone(), &"3_2".to_string()),
        (true, vec!["4_1".to_string(), "4_3".to_string()])
    );
    assert_eq!(
        can_black_pawn_move(game_state.clone(), &"3_4".to_string()),
        (true, vec!["4_3".to_string(), "4_5".to_string()])
    );
    assert_eq!(
        can_black_pawn_move(game_state.clone(), &"3_8".to_string()),
        (true, vec!["4_7".to_string()])
    );

    assert_eq!(
        can_black_pawn_move(game_state.clone(), &"2_1".to_string()),
        (false, vec!["".to_string()])
    );
    assert_eq!(
        can_black_pawn_move(game_state.clone(), &"2_3".to_string()),
        (false, vec!["".to_string()])
    );
    assert_eq!(
        can_black_pawn_move(game_state.clone(), &"2_7".to_string()),
        (false, vec!["".to_string()])
    );
}

#[test]
fn test_can_white_pawn_move() {
    let game_state = GameState {
        player: Player::White,
        board_state: get_start_board(),
    };
    assert_eq!(
        can_white_pawn_move(game_state.clone(), &"6_1".to_string()),
        (true, vec!["5_2".to_string()])
    );
    assert_eq!(
        can_white_pawn_move(game_state.clone(), &"6_3".to_string()),
        (true, vec!["5_2".to_string(), "5_4".to_string()])
    );
    assert_eq!(
        can_white_pawn_move(game_state.clone(), &"6_7".to_string()),
        (true, vec!["5_6".to_string(), "5_8".to_string()])
    );
    assert_eq!(
        can_white_pawn_move(game_state.clone(), &"7_2".to_string()),
        (false, vec!["".to_string()])
    );
    assert_eq!(
        can_white_pawn_move(game_state.clone(), &"7_4".to_string()),
        (false, vec!["".to_string()])
    );
    assert_eq!(
        can_white_pawn_move(game_state.clone(), &"7_8".to_string()),
        (false, vec!["".to_string()])
    );
}

#[test]
fn test_can_black_pawn_beat() {
    let mut board_state = get_start_board();

    board_state.entry("6_1".to_string()).and_modify(|k| {
        *k = FieldState {
            pawn_color: PawnColor::Empty,
            pawn_type: PawnType::Empty,
        }
    });

    board_state.entry("3_4".to_string()).and_modify(|k| {
        *k = FieldState {
            pawn_color: PawnColor::Empty,
            pawn_type: PawnType::Empty,
        }
    });

    board_state.entry("4_3".to_string()).and_modify(|k| {
        *k = FieldState {
            pawn_color: PawnColor::Black,
            pawn_type: PawnType::Pawn,
        }
    });

    board_state.entry("5_2".to_string()).and_modify(|k| {
        *k = FieldState {
            pawn_color: PawnColor::White,
            pawn_type: PawnType::Pawn,
        }
    });

    let game_state = GameState {
        player: Player::Black,
        board_state: board_state,
    };

    let result = can_black_pawn_beat(game_state, &"4_3".to_string());
    let expeced = (true, vec![("5_2".to_string(), "6_1".to_string())]);

    assert_eq!(result, expeced);
}

#[test]
fn test_can_white_pawn_beat() {
    let mut board_state = get_start_board();

    board_state.entry("6_1".to_string()).and_modify(|k| {
        *k = FieldState {
            pawn_color: PawnColor::Empty,
            pawn_type: PawnType::Empty,
        }
    });

    board_state.entry("3_4".to_string()).and_modify(|k| {
        *k = FieldState {
            pawn_color: PawnColor::Empty,
            pawn_type: PawnType::Empty,
        }
    });

    board_state.entry("4_3".to_string()).and_modify(|k| {
        *k = FieldState {
            pawn_color: PawnColor::Black,
            pawn_type: PawnType::Pawn,
        }
    });

    board_state.entry("5_2".to_string()).and_modify(|k| {
        *k = FieldState {
            pawn_color: PawnColor::White,
            pawn_type: PawnType::Pawn,
        }
    });

    let game_state = GameState {
        player: Player::White,
        board_state: board_state,
    };

    let result = can_white_pawn_beat(game_state, &"5_2".to_string());
    let expeced = (true, vec![("4_3".to_string(), "3_4".to_string())]);

    assert_eq!(result, expeced);
}

#[test]
fn test_get_avaiable_actions_black() {
    let board_state = get_start_board();

    let avaiable_actions = get_avaiable_actions(GameState {
        player: Player::Black,
        board_state,
    });

    assert_eq!(avaiable_actions.pawns_can_beat.len(), 0);
    assert_eq!(avaiable_actions.dames_can_move.len(), 0);
    assert_eq!(avaiable_actions.dames_can_beat.len(), 0);

    assert_eq!(
        avaiable_actions.pawns_can_move.get("3_2").unwrap(),
        &vec!["4_1".to_string(), "4_3".to_string()]
    );
    assert_eq!(
        avaiable_actions.pawns_can_move.get("3_4").unwrap(),
        &vec!["4_3".to_string(), "4_5".to_string()]
    );
    assert_eq!(
        avaiable_actions.pawns_can_move.get("3_6").unwrap(),
        &vec!["4_5".to_string(), "4_7".to_string()]
    );
    assert_eq!(
        avaiable_actions.pawns_can_move.get("3_8").unwrap(),
        &vec!["4_7".to_string()]
    );
}

#[test]
fn test_get_avaiable_actions_white() {
    let board_state = get_start_board();

    let avaiable_actions = get_avaiable_actions(GameState {
        player: Player::White,
        board_state,
    });

    assert_eq!(avaiable_actions.pawns_can_beat.len(), 0);
    assert_eq!(avaiable_actions.dames_can_move.len(), 0);
    assert_eq!(avaiable_actions.dames_can_beat.len(), 0);

    assert_eq!(
        avaiable_actions.pawns_can_move.get("6_1").unwrap(),
        &vec!["5_2".to_string()]
    );
    assert_eq!(
        avaiable_actions.pawns_can_move.get("6_3").unwrap(),
        &vec!["5_2".to_string(), "5_4".to_string()]
    );
    assert_eq!(
        avaiable_actions.pawns_can_move.get("6_5").unwrap(),
        &vec!["5_4".to_string(), "5_6".to_string()]
    );
    assert_eq!(
        avaiable_actions.pawns_can_move.get("6_7").unwrap(),
        &vec!["5_6".to_string(), "5_8".to_string()]
    );
}

#[test]
fn test_is_position_free() {
    let board_state = get_start_board();
    assert!(is_position_free(&board_state, &"5_2".to_string()));
    assert!(!is_position_free(&board_state, &"6_1".to_string()));
}

// TODO dame, random move, 'best" move
