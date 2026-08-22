use rand::prelude::IndexedRandom;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::collections::{hash_map::Entry, HashMap};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn opposite(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
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

pub enum ActionType {
    PawnMove,
    PawnBeat,
    DameMove,
    DameBeat,
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
pub struct AvailableActions {
    pub pawns_can_move: HashMap<String, Vec<String>>,
    pub pawns_can_beat: HashMap<String, Vec<(String, String)>>,
    pub dames_can_move: HashMap<String, Vec<String>>,
    pub dames_can_beat: HashMap<String, Vec<(String, String)>>,
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

fn promote_pawn(moved_pawn: FieldState, destination: &str) -> FieldState {
    if moved_pawn.pawn_type != PawnType::Pawn {
        return moved_pawn;
    };
    if (moved_pawn.pawn_color == PawnColor::Black && destination.starts_with("8_"))
        || (moved_pawn.pawn_color == PawnColor::White && destination.starts_with("1_"))
    {
        FieldState {
            pawn_type: PawnType::Dame,
            ..moved_pawn
        }
    } else {
        moved_pawn
    }
}

pub fn make_move(game_state: GameState, start: String, destination: String) -> GameState {
    let mut new_game_state = game_state.clone();
    let dest_field = new_game_state.board_state.get_mut(&destination).unwrap();
    let moved_pawn = game_state.board_state.get(&start).unwrap().clone();

    *dest_field = promote_pawn(moved_pawn, &*destination);

    let start_field = new_game_state.board_state.get_mut(&start).unwrap();
    *start_field = FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    };
    new_game_state
}
pub fn beat(game_state: GameState, start: String, destination: (String, String)) -> GameState {
    let mut state = game_state;
    let (captured, landing) = destination;
    let moved_piece = state.board_state.get(&start).unwrap().clone();

    state
        .board_state
        .insert(landing.clone(), promote_pawn(moved_piece, &landing));

    state.board_state.insert(
        captured,
        FieldState {
            pawn_color: PawnColor::Empty,
            pawn_type: PawnType::Empty,
        },
    );
    state.board_state.insert(
        start,
        FieldState {
            pawn_color: PawnColor::Empty,
            pawn_type: PawnType::Empty,
        },
    );

    state
}
fn get_piece_captures(game_state: &GameState, position: &str) -> Vec<(String, String)> {
    let field = game_state.board_state.get(position).unwrap();
    let pawn_color = match game_state.player {
        Player::Black => PawnColor::Black,
        Player::White => PawnColor::White,
    };
    let captures = match field.pawn_type {
        PawnType::Pawn => {
            let result = can_pawn_beat(&game_state, &position.to_string(), &pawn_color);

            if result.0 {
                result.1
            } else {
                Vec::new()
            }
        }
        PawnType::Dame => {
            let result = can_dame_beat(game_state.clone(), &position.to_string());

            if result.0 {
                result.1
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    };

    captures
}
pub fn generate_capture_sequences(
    game_state: GameState,
    position: String,
    states: &mut Vec<GameState>,
) {
    let captures = get_piece_captures(&game_state, &position);
    if captures.is_empty() {
        states.push(game_state);
        return;
    }
    for capture in captures {
        let next_position = capture.1.clone();
        let next_state = beat(game_state.clone(), position.clone(), capture);
        generate_capture_sequences(next_state, next_position, states);
    }
}

pub fn make_random_move(game_state: GameState) -> Option<GameState> {
    let available_actions: AvailableActions = get_available_actions(&game_state);
    let mut available_actions_types: Vec<ActionType> = Vec::new();

    let mut new_game_state = game_state.clone();
    let mut can_meke_move = false;

    if available_actions.pawns_can_move.len() > 0 {
        available_actions_types.push(ActionType::PawnMove);
    };
    if available_actions.pawns_can_beat.len() > 0 {
        available_actions_types.push(ActionType::PawnBeat);
    };
    if available_actions.dames_can_move.len() > 0 {
        available_actions_types.push(ActionType::DameMove);
    };
    if available_actions.dames_can_beat.len() > 0 {
        available_actions_types.push(ActionType::DameBeat);
    };
    match available_actions_types.into_iter().choose(&mut rand::rng()) {
        Some(action) => {
            can_meke_move = true;

            match action {
                ActionType::PawnMove => {
                    let pawn_move = available_actions
                        .pawns_can_move
                        .into_iter()
                        .choose(&mut rand::rng())
                        .unwrap();
                    let start = pawn_move.0;
                    let destination = pawn_move.1.choose(&mut rand::rng())?.to_owned();
                    new_game_state = make_move(game_state, start, destination);
                }
                ActionType::PawnBeat => {
                    let pawn_beat = available_actions
                        .pawns_can_beat
                        .into_iter()
                        .choose(&mut rand::rng())
                        .unwrap();
                    let start = pawn_beat.0;
                    let destination = pawn_beat.1.choose(&mut rand::rng())?.to_owned();

                    new_game_state = beat(game_state, start, destination)
                }
                ActionType::DameMove => {
                    let dame_move = available_actions
                        .dames_can_move
                        .into_iter()
                        .choose(&mut rand::rng())
                        .unwrap();
                    let start = dame_move.0;
                    let destination = dame_move.1.choose(&mut rand::rng())?.to_string();
                    new_game_state = make_move(game_state, start, destination);
                }
                ActionType::DameBeat => {
                    let dame_beat = available_actions
                        .dames_can_beat
                        .into_iter()
                        .choose(&mut rand::rng())
                        .unwrap();
                    let start = dame_beat.0;
                    let destination = dame_beat.1.choose(&mut rand::rng())?.to_owned();

                    new_game_state = beat(game_state, start, destination)
                }
            }
        }
        None => {}
    };
    if can_meke_move {
        Some(new_game_state)
    } else {
        None
    }
}
pub fn get_available_actions(game_state: &GameState) -> AvailableActions {
    let mut pawns_can_move: HashMap<String, Vec<String>> = HashMap::new();
    let mut pawns_can_beat: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut dames_can_move: HashMap<String, Vec<String>> = HashMap::new();
    let mut dames_can_beat: HashMap<String, Vec<(String, String)>> = HashMap::new();

    let pawn_color = match game_state.player {
        Player::Black => PawnColor::Black,
        Player::White => PawnColor::White,
    };
    game_state
        .clone()
        .board_state
        .into_iter()
        .for_each(|state| {
            if state.1.pawn_color != pawn_color {
                return;
            }
            match state.1 {
                FieldState {
                    pawn_color: _,
                    pawn_type: PawnType::Pawn,
                } => {
                    let moves = can_pawn_move(game_state, &state.0, &pawn_color);
                    if !moves.is_empty() {
                        match pawns_can_move.entry(state.0.clone()) {
                            Entry::Vacant(e) => {
                                e.insert(moves.clone());
                            }
                            Entry::Occupied(mut e) => {
                                e.get_mut().extend(moves.clone());
                            }
                        }
                    }
                    let can_beat = can_pawn_beat(game_state, &state.0, &pawn_color);
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
                    pawn_color: _,
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
            }
        });
    AvailableActions {
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

fn can_pawn_move(game_state: &GameState, position: &str, pawn_color: &PawnColor) -> Vec<String> {
    let parts: Vec<&str> = position.split('_').collect();

    let x: i32 = parts[0].parse().expect("not a number");
    let y: i32 = parts[1].parse().expect("not a number");

    let dx = match pawn_color {
        PawnColor::Black => 1,
        PawnColor::White => -1,
        PawnColor::Empty => return Vec::new(),
    };
    let mut moves = Vec::new();
    for dy in [-1, 1] {
        let nx = x + dx;
        let ny = y + dy;

        if !(1..=8).contains(&nx) || !(1..=8).contains(&ny) {
            continue;
        }

        let next_position = format!("{}_{}", nx, ny);
        if is_position_free(&game_state.board_state, &next_position) {
            moves.push(next_position);
        }
    }
    moves
}

fn can_pawn_beat(
    game_state: &GameState,
    position: &String,
    pawn_color: &PawnColor,
) -> (bool, Vec<(String, String)>) {
    let parts: Vec<&str> = position.split('_').collect();

    let x: i32 = parts[0].parse().expect("not a number");
    let y: i32 = parts[1].parse().expect("not a number");

    let (my_color, opponent_color, dx) = match pawn_color {
        PawnColor::Black => (PawnColor::Black, PawnColor::White, 1),
        PawnColor::White => (PawnColor::White, PawnColor::Black, -1),
        PawnColor::Empty => {
            unreachable!()
        }
    };
    let mut captures = Vec::new();
    for dy in [-1, 1] {
        let enemy_x = x + dx;
        let enemy_y = y + dy;

        let landing_x = x + 2 * dx;
        let landing_y = y + 2 * dy;

        // Check board boundaries.
        if !(0..8).contains(&enemy_x)
            || !(0..8).contains(&enemy_y)
            || !(0..8).contains(&landing_x)
            || !(0..8).contains(&landing_y)
        {
            continue;
        }

        let enemy_position = format!("{}_{}", enemy_x, enemy_y);
        let landing_position = format!("{}_{}", landing_x, landing_y);
        // There must be an opponent on the middle square.
        let is_opponent = matches!(
            game_state.board_state.get(&enemy_position),
            Some(FieldState {
                pawn_color,
                pawn_type: PawnType::Pawn | PawnType::Dame,
            }) if *pawn_color == opponent_color
        );

        if !is_opponent {
            continue;
        }
        // Landing square must be empty.
        if is_position_free(&game_state.board_state, &landing_position) {
            captures.push((enemy_position, landing_position));
        }
    }
    if captures.is_empty() {
        (false, Vec::new())
    } else {
        (true, captures)
    }
}

fn can_dame_move(game_state: GameState, position: &String) -> (bool, Vec<String>) {
    let parts: Vec<&str> = position.split('_').collect();

    let x: i32 = parts[0].parse().expect("not a number");
    let y: i32 = parts[1].parse().expect("not a number");

    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    let mut destinations = Vec::new();
    for (dx, dy) in directions {
        let mut nx = x + dx;
        let mut ny = y + dy;

        while (0..8).contains(&nx) && (0..8).contains(&ny) {
            let position = format!("{}_{}", nx, ny);

            if !is_position_free(&game_state.board_state, &position) {
                break;
            }
            destinations.push(position);
            nx += dx;
            ny += dy;
        }
    }
    if destinations.is_empty() {
        (false, Vec::new())
    } else {
        (true, destinations)
    }
}
fn can_dame_beat(game_state: GameState, position: &String) -> (bool, Vec<(String, String)>) {
    let parts: Vec<&str> = position.split('_').collect();

    let x: i32 = parts[0].parse().expect("not a number");
    let y: i32 = parts[1].parse().expect("not a number");

    let my_color = match game_state.player {
        Player::Black => PawnColor::Black,
        Player::White => PawnColor::White,
    };

    let opponent_color = match game_state.player {
        Player::Black => PawnColor::White,
        Player::White => PawnColor::Black,
    };

    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    let mut captures = Vec::new();

    for (dx, dy) in directions {
        let mut nx = x + dx;
        let mut ny = y + dy;

        // Before encountering an opponent, there may only be
        // empty squares.
        while (0..8).contains(&nx) && (0..8).contains(&ny) {
            let current_position = format!("{}_{}", nx, ny);

            match game_state.board_state.get(&current_position) {
                None => {
                    break;
                }
                Some(FieldState {
                    pawn_color: PawnColor::Empty,
                    ..
                }) => {
                    nx += dx;
                    ny += dy;
                }
                Some(FieldState { pawn_color, .. }) if *pawn_color == my_color => {
                    // Own piece blocks this direction.
                    break;
                }
                Some(FieldState { pawn_color, .. }) if *pawn_color == opponent_color => {
                    // Found an opponent.
                    let enemy_position = current_position;

                    // Now look for every possible landing square
                    // beyond the opponent.
                    nx += dx;
                    ny += dy;

                    while (0..8).contains(&nx) && (0..8).contains(&ny) {
                        let landing_position = format!("{}_{}", nx, ny);

                        match game_state.board_state.get(&landing_position) {
                            Some(FieldState {
                                pawn_color: PawnColor::Empty,
                                ..
                            }) => {
                                captures.push((enemy_position.clone(), landing_position));
                            }
                            _ => {
                                // Any piece beyond the captured
                                // piece blocks further landing squares.
                                break;
                            }
                        }

                        nx += dx;
                        ny += dy;
                    }
                    // We can encounter only one enemy in this
                    // direction during a single jump.
                    break;
                }

                _ => break,
            }
        }
    }
    if captures.is_empty() {
        (false, Vec::new())
    } else {
        (true, captures)
    }
}

#[test]
fn test_can_black_pawn_move() {
    let game_state = GameState {
        player: Player::Black,
        board_state: get_start_board(),
    };
    assert_eq!(
        can_pawn_move(&game_state, &"3_2".to_string(), &PawnColor::Black),
        vec!["4_1".to_string(), "4_3".to_string()]
    );
    assert_eq!(
        can_pawn_move(&game_state, &"3_4".to_string(), &PawnColor::Black),
        vec!["4_3".to_string(), "4_5".to_string()]
    );
    assert_eq!(
        can_pawn_move(&game_state, &"3_8".to_string(), &PawnColor::Black),
        vec!["4_7".to_string()]
    );
    assert!(can_pawn_move(&game_state, &"2_1".to_string(), &PawnColor::Black).is_empty());
    assert!(can_pawn_move(&game_state, &"2_3".to_string(), &PawnColor::Black).is_empty());
    assert!(can_pawn_move(&game_state, &"2_7".to_string(), &PawnColor::Black).is_empty());
}

#[test]
fn test_can_white_pawn_move() {
    let game_state = GameState {
        player: Player::White,
        board_state: get_start_board(),
    };
    assert_eq!(
        can_pawn_move(&game_state, &"6_1".to_string(), &PawnColor::White),
        vec!["5_2".to_string()]
    );
    assert_eq!(
        can_pawn_move(&game_state, &"6_3".to_string(), &PawnColor::White),
        vec!["5_2".to_string(), "5_4".to_string()]
    );
    assert_eq!(
        can_pawn_move(&game_state, &"6_7".to_string(), &PawnColor::White),
        vec!["5_6".to_string(), "5_8".to_string()]
    );
    assert!(can_pawn_move(&game_state, &"7_2".to_string(), &PawnColor::White).is_empty());
    assert!(can_pawn_move(&game_state, &"7_4".to_string(), &PawnColor::White).is_empty(),);
    assert!(can_pawn_move(&game_state, &"7_8".to_string(), &PawnColor::White).is_empty(),);
}

#[test]
fn test_get_available_actions_color_filter() {
    let board_state = get_start_board();
    // Black's turn, but let's see if it only picks black pawns
    let game_state = GameState {
        player: Player::Black,
        board_state,
    };
    let actions = get_available_actions(&game_state);

    for (pos, _) in actions.pawns_can_move {
        let field = game_state.board_state.get(&pos).unwrap();
        assert_eq!(field.pawn_color, PawnColor::Black);
    }
    for (pos, _) in actions.pawns_can_beat {
        let field = game_state.board_state.get(&pos).unwrap();
        assert_eq!(field.pawn_color, PawnColor::Black);
    }
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
        board_state,
    };

    let result = can_pawn_beat(&game_state, &"4_3".to_string(), &PawnColor::Black);
    let expected = (true, vec![("5_2".to_string(), "6_1".to_string())]);

    assert_eq!(result, expected);
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
        board_state,
    };

    let result = can_pawn_beat(&game_state, &"5_2".to_string(), &PawnColor::White);
    let expected = (true, vec![("4_3".to_string(), "3_4".to_string())]);

    assert_eq!(result, expected);
}

#[test]
fn test_get_available_actions_black() {
    let board_state = get_start_board();

    let available_actions = get_available_actions(&GameState {
        player: Player::Black,
        board_state,
    });

    assert_eq!(available_actions.pawns_can_beat.len(), 0);
    assert_eq!(available_actions.dames_can_move.len(), 0);
    assert_eq!(available_actions.dames_can_beat.len(), 0);

    assert_eq!(
        available_actions.pawns_can_move.get("3_2").unwrap(),
        &vec!["4_1".to_string(), "4_3".to_string()]
    );
    assert_eq!(
        available_actions.pawns_can_move.get("3_4").unwrap(),
        &vec!["4_3".to_string(), "4_5".to_string()]
    );
    assert_eq!(
        available_actions.pawns_can_move.get("3_6").unwrap(),
        &vec!["4_5".to_string(), "4_7".to_string()]
    );
    assert_eq!(
        available_actions.pawns_can_move.get("3_8").unwrap(),
        &vec!["4_7".to_string()]
    );
}

#[test]
fn test_get_available_actions_white() {
    let board_state = get_start_board();

    let available_actions = get_available_actions(&GameState {
        player: Player::White,
        board_state,
    });

    assert_eq!(available_actions.pawns_can_beat.len(), 0);
    assert_eq!(available_actions.dames_can_move.len(), 0);
    assert_eq!(available_actions.dames_can_beat.len(), 0);

    assert_eq!(
        available_actions.pawns_can_move.get("6_1").unwrap(),
        &vec!["5_2".to_string()]
    );
    assert_eq!(
        available_actions.pawns_can_move.get("6_3").unwrap(),
        &vec!["5_2".to_string(), "5_4".to_string()]
    );
    assert_eq!(
        available_actions.pawns_can_move.get("6_5").unwrap(),
        &vec!["5_4".to_string(), "5_6".to_string()]
    );
    assert_eq!(
        available_actions.pawns_can_move.get("6_7").unwrap(),
        &vec!["5_6".to_string(), "5_8".to_string()]
    );
}

#[test]
fn test_is_position_free() {
    let board_state = get_start_board();
    assert!(is_position_free(&board_state, &"5_2".to_string()));
    assert!(!is_position_free(&board_state, &"6_1".to_string()));
}

#[test]
fn test_pawn_promotion() {
    let mut board_state: HashMap<String, FieldState> = HashMap::new();
    for i in 1..9 {
        for j in 1..9 {
            board_state.insert(
                format!("{}_{}", i, j),
                FieldState {
                    pawn_color: PawnColor::Empty,
                    pawn_type: PawnType::Empty,
                },
            );
        }
    }

    // Black pawn at row 7, moving to row 8
    board_state.insert(
        "7_2".to_string(),
        FieldState {
            pawn_color: PawnColor::Black,
            pawn_type: PawnType::Pawn,
        },
    );

    let game_state = GameState {
        player: Player::Black,
        board_state,
    };

    let new_game_state = make_move(game_state, "7_2".to_string(), "8_1".to_string());
    let promoted_pawn = new_game_state.board_state.get("8_1").unwrap();

    assert_eq!(promoted_pawn.pawn_type, PawnType::Dame);
    assert_eq!(promoted_pawn.pawn_color, PawnColor::Black);
}

#[test]
fn test_pawn_promotion_beat() {
    let mut board_state: HashMap<String, FieldState> = HashMap::new();
    for i in 1..9 {
        for j in 1..9 {
            board_state.insert(
                format!("{}_{}", i, j),
                FieldState {
                    pawn_color: PawnColor::Empty,
                    pawn_type: PawnType::Empty,
                },
            );
        }
    }

    // Black pawn at row 7, beating to row 8
    board_state.insert(
        "6_3".to_string(),
        FieldState {
            pawn_color: PawnColor::Black,
            pawn_type: PawnType::Pawn,
        },
    );
    board_state.insert(
        "7_2".to_string(),
        FieldState {
            pawn_color: PawnColor::White,
            pawn_type: PawnType::Pawn,
        },
    );

    let game_state = GameState {
        player: Player::Black,
        board_state,
    };

    let new_game_state = beat(
        game_state,
        "6_3".to_string(),
        ("7_2".to_string(), "8_1".to_string()),
    );
    let promoted_pawn = new_game_state.board_state.get("8_1").unwrap();

    assert_eq!(promoted_pawn.pawn_type, PawnType::Dame);
    assert_eq!(promoted_pawn.pawn_color, PawnColor::Black);
    // Ensure the beaten pawn is gone
    assert_eq!(
        new_game_state.board_state.get("7_2").unwrap().pawn_type,
        PawnType::Empty
    );
}
