use rand::prelude::IndexedRandom;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

fn serialize_board<S>(board: &[FieldState; 64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = HashMap::new();
    for i in 0..64 {
        let field = &board[i];
        if field.pawn_color != PawnColor::Empty {
            let row = i / 8 + 1;
            let col = i % 8 + 1;
            let key = format!("{}_{}", row, col);
            map.insert(key, field);
        }
    }
    map.serialize(serializer)
}

fn deserialize_board<'de, D>(deserializer: D) -> Result<[FieldState; 64], D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, FieldState> = HashMap::deserialize(deserializer)?;
    let mut board = [FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    }; 64];

    for (key, field) in map {
        let parts: Vec<&str> = key.split('_').collect();
        if parts.len() == 2 {
            if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                if row >= 1 && row <= 8 && col >= 1 && col <= 8 {
                    let idx = (row - 1) * 8 + (col - 1);
                    board[idx] = field;
                }
            }
        }
    }
    Ok(board)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PawnColor {
    Empty,
    Black,
    White,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldState {
    pub pawn_color: PawnColor,
    pub pawn_type: PawnType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    pub player: Player,
    #[serde(
        serialize_with = "serialize_board",
        deserialize_with = "deserialize_board"
    )]
    pub board_state: [FieldState; 64],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvailableActions {
    pub pawns_can_move: HashMap<usize, Vec<usize>>,
    pub pawns_can_beat: HashMap<usize, Vec<(usize, usize)>>,
    pub dames_can_move: HashMap<usize, Vec<usize>>,
    pub dames_can_beat: HashMap<usize, Vec<(usize, usize)>>,
}

pub fn get_start_board() -> [FieldState; 64] {
    let mut board = [FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    }; 64];

    for i in 1..9 {
        for j in 1..9 {
            let idx = (i - 1) * 8 + (j - 1);
            if (i == 1 || i == 3) && j % 2 == 0 {
                board[idx] = FieldState {
                    pawn_color: PawnColor::Black,
                    pawn_type: PawnType::Pawn,
                };
            } else if i == 2 && j % 2 != 0 {
                board[idx] = FieldState {
                    pawn_color: PawnColor::Black,
                    pawn_type: PawnType::Pawn,
                };
            } else if (i == 6 || i == 8) && j % 2 != 0 {
                board[idx] = FieldState {
                    pawn_color: PawnColor::White,
                    pawn_type: PawnType::Pawn,
                };
            } else if i == 7 && j % 2 == 0 {
                board[idx] = FieldState {
                    pawn_color: PawnColor::White,
                    pawn_type: PawnType::Pawn,
                };
            }
        }
    }
    board
}

pub fn promote_pawn(moved_pawn: FieldState, destination_idx: usize) -> FieldState {
    if moved_pawn.pawn_type != PawnType::Pawn {
        return moved_pawn;
    }
    let row = destination_idx / 8 + 1;
    if (moved_pawn.pawn_color == PawnColor::Black && row == 8)
        || (moved_pawn.pawn_color == PawnColor::White && row == 1)
    {
        FieldState {
            pawn_type: PawnType::Dame,
            ..moved_pawn
        }
    } else {
        moved_pawn
    }
}

pub fn make_move(mut game_state: GameState, start: usize, destination: usize) -> GameState {
    let moved_pawn = game_state.board_state[start];
    game_state.board_state[destination] = promote_pawn(moved_pawn, destination);
    game_state.board_state[start] = FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    };
    game_state
}

pub fn beat(mut game_state: GameState, start: usize, destination: (usize, usize)) -> GameState {
    let (captured, landing) = destination;
    let moved_piece = game_state.board_state[start];
    game_state.board_state[landing] = promote_pawn(moved_piece, landing);
    game_state.board_state[captured] = FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    };
    game_state.board_state[start] = FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    };
    game_state
}

fn get_piece_captures(game_state: &GameState, position_idx: usize) -> Vec<(usize, usize)> {
    let field = &game_state.board_state[position_idx];
    let pawn_color = match game_state.player {
        Player::Black => PawnColor::Black,
        Player::White => PawnColor::White,
    };
    if field.pawn_type == PawnType::Pawn {
        let (_, captures) = can_pawn_beat(game_state, position_idx, &pawn_color);
        captures
    } else if field.pawn_type == PawnType::Dame {
        let (_, captures) = can_dame_beat(game_state, position_idx);
        captures
    } else {
        Vec::new()
    }
}

pub fn generate_capture_sequences(
    game_state: GameState,
    position_idx: usize,
    states: &mut Vec<GameState>,
) {
    let captures = get_piece_captures(&game_state, position_idx);
    if captures.is_empty() {
        states.push(game_state);
        return;
    }
    for capture in captures {
        let next_position = capture.1;
        let next_state = beat(game_state.clone(), position_idx, capture);
        generate_capture_sequences(next_state, next_position, states);
    }
}

pub fn make_random_move(game_state: GameState) -> Option<GameState> {
    let available_actions: AvailableActions = get_available_actions(&game_state);
    let mut available_actions_types: Vec<ActionType> = Vec::new();

    if !available_actions.pawns_can_move.is_empty() {
        available_actions_types.push(ActionType::PawnMove);
    };
    if !available_actions.pawns_can_beat.is_empty() {
        available_actions_types.push(ActionType::PawnBeat);
    };
    if !available_actions.dames_can_move.is_empty() {
        available_actions_types.push(ActionType::DameMove);
    };
    if !available_actions.dames_can_beat.is_empty() {
        available_actions_types.push(ActionType::DameBeat);
    };

    let action = available_actions_types
        .into_iter()
        .choose(&mut rand::rng())?;

    match action {
        ActionType::PawnMove => {
            let (&start, destinations) = available_actions
                .pawns_can_move
                .iter()
                .choose(&mut rand::rng())?;
            let &destination = destinations.choose(&mut rand::rng())?;
            Some(make_move(game_state, start, destination))
        }
        ActionType::PawnBeat => {
            let (&start, captures) = available_actions
                .pawns_can_beat
                .iter()
                .choose(&mut rand::rng())?;
            let &destination = captures.choose(&mut rand::rng())?;
            Some(beat(game_state, start, destination))
        }
        ActionType::DameMove => {
            let (&start, destinations) = available_actions
                .dames_can_move
                .iter()
                .choose(&mut rand::rng())?;
            let &destination = destinations.choose(&mut rand::rng())?;
            Some(make_move(game_state, start, destination))
        }
        ActionType::DameBeat => {
            let (&start, captures) = available_actions
                .dames_can_beat
                .iter()
                .choose(&mut rand::rng())?;
            let &destination = captures.choose(&mut rand::rng())?;
            Some(beat(game_state, start, destination))
        }
    }
}

pub fn get_available_actions(game_state: &GameState) -> AvailableActions {
    let mut pawns_can_move = HashMap::new();
    let mut pawns_can_beat = HashMap::new();
    let mut dames_can_move = HashMap::new();
    let mut dames_can_beat = HashMap::new();

    let current_pawn_color = match game_state.player {
        Player::Black => PawnColor::Black,
        Player::White => PawnColor::White,
    };

    for idx in 0..64 {
        let field = &game_state.board_state[idx];
        if field.pawn_color != current_pawn_color {
            continue;
        }

        if field.pawn_type == PawnType::Pawn {
            let moves = can_pawn_move(game_state, idx, &current_pawn_color);
            if !moves.is_empty() {
                pawns_can_move.insert(idx, moves);
            }
            let (can_beat, captures) = can_pawn_beat(game_state, idx, &current_pawn_color);
            if can_beat {
                pawns_can_beat.insert(idx, captures);
            }
        } else if field.pawn_type == PawnType::Dame {
            let (can_move, moves) = can_dame_move(game_state, idx);
            if can_move {
                dames_can_move.insert(idx, moves);
            }
            let (can_beat, captures) = can_dame_beat(game_state, idx);
            if can_beat {
                dames_can_beat.insert(idx, captures);
            }
        }
    }

    AvailableActions {
        pawns_can_move,
        pawns_can_beat,
        dames_can_move,
        dames_can_beat,
    }
}

fn is_position_free(game_state: &GameState, idx: usize) -> bool {
    game_state.board_state[idx].pawn_type == PawnType::Empty
}

fn can_pawn_move(game_state: &GameState, idx: usize, color: &PawnColor) -> Vec<usize> {
    let x = (idx / 8 + 1) as i32;
    let y = (idx % 8 + 1) as i32;
    let dx = match color {
        PawnColor::Black => 1,
        PawnColor::White => -1,
        _ => return Vec::new(),
    };

    let mut moves = Vec::new();
    for dy in [-1, 1] {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
            let nidx = ((nx - 1) * 8 + (ny - 1)) as usize;
            if is_position_free(game_state, nidx) {
                moves.push(nidx);
            }
        }
    }
    moves
}

fn can_pawn_beat(
    game_state: &GameState,
    idx: usize,
    color: &PawnColor,
) -> (bool, Vec<(usize, usize)>) {
    let x = (idx / 8 + 1) as i32;
    let y = (idx % 8 + 1) as i32;
    let (opponent_color, dx) = match color {
        PawnColor::Black => (PawnColor::White, 1),
        PawnColor::White => (PawnColor::Black, -1),
        _ => return (false, Vec::new()),
    };

    let mut captures = Vec::new();
    for dy in [-1, 1] {
        let ex = x + dx;
        let ey = y + dy;
        let lx = x + 2 * dx;
        let ly = y + 2 * dy;

        if ex >= 1 && ex <= 8 && ey >= 1 && ey <= 8 && lx >= 1 && lx <= 8 && ly >= 1 && ly <= 8 {
            let eidx = ((ex - 1) * 8 + (ey - 1)) as usize;
            let lidx = ((lx - 1) * 8 + (ly - 1)) as usize;
            let efield = &game_state.board_state[eidx];
            if efield.pawn_color == opponent_color && is_position_free(game_state, lidx) {
                captures.push((eidx, lidx));
            }
        }
    }
    (!captures.is_empty(), captures)
}

fn can_dame_move(game_state: &GameState, idx: usize) -> (bool, Vec<usize>) {
    let x = (idx / 8 + 1) as i32;
    let y = (idx % 8 + 1) as i32;
    let mut moves = Vec::new();
    let dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (dx, dy) in dirs {
        let mut nx = x + dx;
        let mut ny = y + dy;
        while nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
            let nidx = ((nx - 1) * 8 + (ny - 1)) as usize;
            if !is_position_free(game_state, nidx) {
                break;
            }
            moves.push(nidx);
            nx += dx;
            ny += dy;
        }
    }
    (!moves.is_empty(), moves)
}

fn can_dame_beat(game_state: &GameState, idx: usize) -> (bool, Vec<(usize, usize)>) {
    let x = (idx / 8 + 1) as i32;
    let y = (idx % 8 + 1) as i32;
    let my_color = match game_state.player {
        Player::Black => PawnColor::Black,
        Player::White => PawnColor::White,
    };
    let opponent_color = match game_state.player {
        Player::Black => PawnColor::White,
        Player::White => PawnColor::Black,
    };
    let mut captures = Vec::new();
    let dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (dx, dy) in dirs {
        let mut nx = x + dx;
        let mut ny = y + dy;
        while nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
            let nidx = ((nx - 1) * 8 + (ny - 1)) as usize;
            let field = &game_state.board_state[nidx];
            if field.pawn_color == my_color {
                break;
            }
            if field.pawn_color == opponent_color {
                let eidx = nidx;
                nx += dx;
                ny += dy;
                while nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
                    let lidx = ((nx - 1) * 8 + (ny - 1)) as usize;
                    if !is_position_free(game_state, lidx) {
                        break;
                    }
                    captures.push((eidx, lidx));
                    nx += dx;
                    ny += dy;
                }
                break;
            }
            nx += dx;
            ny += dy;
        }
    }
    (!captures.is_empty(), captures)
}

#[test]
fn test_can_black_pawn_move() {
    let game_state = GameState {
        player: Player::Black,
        board_state: get_start_board(),
    };
    // 3_2 -> 17, 4_1 -> 24, 4_3 -> 26
    assert_eq!(
        can_pawn_move(&game_state, 17, &PawnColor::Black),
        vec![24, 26]
    );
    // 3_4 -> 19, 4_3 -> 26, 4_5 -> 28
    assert_eq!(
        can_pawn_move(&game_state, 19, &PawnColor::Black),
        vec![26, 28]
    );
    // 3_8 -> 23, 4_7 -> 30
    assert_eq!(can_pawn_move(&game_state, 23, &PawnColor::Black), vec![30]);
    // 2_1 -> 8, 2_3 -> 10, 2_7 -> 14
    assert!(can_pawn_move(&game_state, 8, &PawnColor::Black).is_empty());
    assert!(can_pawn_move(&game_state, 10, &PawnColor::Black).is_empty());
    assert!(can_pawn_move(&game_state, 14, &PawnColor::Black).is_empty());
}

#[test]
fn test_can_white_pawn_move() {
    let game_state = GameState {
        player: Player::White,
        board_state: get_start_board(),
    };
    // 6_1 -> 40, 5_2 -> 33
    assert_eq!(can_pawn_move(&game_state, 40, &PawnColor::White), vec![33]);
    // 6_3 -> 42, 5_2 -> 33, 5_4 -> 35
    assert_eq!(
        can_pawn_move(&game_state, 42, &PawnColor::White),
        vec![33, 35]
    );
    // 6_7 -> 46, 5_6 -> 37, 5_8 -> 39
    assert_eq!(
        can_pawn_move(&game_state, 46, &PawnColor::White),
        vec![37, 39]
    );
    // 7_2 -> 49, 7_4 -> 51, 7_8 -> 55
    assert!(can_pawn_move(&game_state, 49, &PawnColor::White).is_empty());
    assert!(can_pawn_move(&game_state, 51, &PawnColor::White).is_empty());
    assert!(can_pawn_move(&game_state, 55, &PawnColor::White).is_empty());
}

#[test]
fn test_get_available_actions_color_filter() {
    let board_state = get_start_board();
    let game_state = GameState {
        player: Player::Black,
        board_state,
    };
    let actions = get_available_actions(&game_state);

    for (pos, _) in actions.pawns_can_move {
        let field = &game_state.board_state[pos];
        assert_eq!(field.pawn_color, PawnColor::Black);
    }
    for (pos, _) in actions.pawns_can_beat {
        let field = &game_state.board_state[pos];
        assert_eq!(field.pawn_color, PawnColor::Black);
    }
}

#[test]
fn test_can_black_pawn_beat() {
    let mut board_state = get_start_board();

    // Empty 6_1 (idx 40)
    board_state[40] = FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    };

    // 4_3 (idx 26) is Black Pawn
    board_state[26] = FieldState {
        pawn_color: PawnColor::Black,
        pawn_type: PawnType::Pawn,
    };

    // 5_2 (idx 33) is White Pawn
    board_state[33] = FieldState {
        pawn_color: PawnColor::White,
        pawn_type: PawnType::Pawn,
    };

    let game_state = GameState {
        player: Player::Black,
        board_state,
    };

    let result = can_pawn_beat(&game_state, 26, &PawnColor::Black);
    let expected = (true, vec![(33, 40)]);

    assert_eq!(result, expected);
}

#[test]
fn test_can_white_pawn_beat() {
    let mut board_state = get_start_board();

    // 3_4 (idx 19) Empty
    board_state[19] = FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    };

    // 4_3 (idx 26) Black Pawn
    board_state[26] = FieldState {
        pawn_color: PawnColor::Black,
        pawn_type: PawnType::Pawn,
    };

    // 5_2 (idx 33) White Pawn
    board_state[33] = FieldState {
        pawn_color: PawnColor::White,
        pawn_type: PawnType::Pawn,
    };

    let game_state = GameState {
        player: Player::White,
        board_state,
    };

    let result = can_pawn_beat(&game_state, 33, &PawnColor::White);
    let expected = (true, vec![(26, 19)]);

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

    // 3_2 (idx 17) -> 4_1(24), 4_3(26)
    assert_eq!(
        available_actions.pawns_can_move.get(&17).unwrap(),
        &vec![24, 26]
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

    // 6_1 (idx 40) -> 5_2(33)
    assert_eq!(
        available_actions.pawns_can_move.get(&40).unwrap(),
        &vec![33]
    );
}

#[test]
fn test_is_position_free() {
    let game_state = GameState {
        player: Player::Black,
        board_state: get_start_board(),
    };
    // 5_2 -> 4*8 + 1 = 33
    assert!(is_position_free(&game_state, 33));
    // 6_1 -> 5*8 + 0 = 40
    assert!(!is_position_free(&game_state, 40));
}

#[test]
fn test_pawn_promotion() {
    let mut board_state = [FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    }; 64];

    // Black pawn at row 7, col 2 (idx 6*8+1=49), moving to row 8, col 1 (idx 7*8+0=56)
    board_state[49] = FieldState {
        pawn_color: PawnColor::Black,
        pawn_type: PawnType::Pawn,
    };

    let game_state = GameState {
        player: Player::Black,
        board_state,
    };

    let new_game_state = make_move(game_state, 49, 56);
    let promoted_pawn = &new_game_state.board_state[56];

    assert_eq!(promoted_pawn.pawn_type, PawnType::Dame);
    assert_eq!(promoted_pawn.pawn_color, PawnColor::Black);
}

#[test]
fn test_pawn_promotion_beat() {
    let mut board_state = [FieldState {
        pawn_color: PawnColor::Empty,
        pawn_type: PawnType::Empty,
    }; 64];

    // Black pawn at row 6, col 3 (idx 5*8+2=42), beating to row 8, col 1 (idx 56) via 7_2 (idx 49)
    board_state[42] = FieldState {
        pawn_color: PawnColor::Black,
        pawn_type: PawnType::Pawn,
    };
    board_state[49] = FieldState {
        pawn_color: PawnColor::White,
        pawn_type: PawnType::Pawn,
    };

    let game_state = GameState {
        player: Player::Black,
        board_state,
    };

    let new_game_state = beat(game_state, 42, (49, 56));
    let promoted_pawn = &new_game_state.board_state[56];

    assert_eq!(promoted_pawn.pawn_type, PawnType::Dame);
    assert_eq!(promoted_pawn.pawn_color, PawnColor::Black);
    assert_eq!(new_game_state.board_state[49].pawn_type, PawnType::Empty);
}

#[test]
fn test_serialization() {
    let game_state = GameState {
        player: Player::Black,
        board_state: get_start_board(),
    };
    let serialized = serde_json::to_string(&game_state).unwrap();
    // Check if it contains "1_2" which is a black pawn in the start board
    assert!(serialized.contains("\"1_2\""));

    let deserialized: GameState = serde_json::from_str(&serialized).unwrap();
    assert_eq!(game_state, deserialized);
}
