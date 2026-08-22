use crate::game::{
    beat, generate_capture_sequences, get_available_actions, make_move, AvailableActions,
    GameState, PawnColor, PawnType, Player,
};
const MATE_SCORE: i32 = 1_000_000;

pub fn get_best_move(game_state: &GameState, depth: i32) -> Option<GameState> {
    let actions = get_available_actions(game_state);
    let all_moves = get_all_possible_states(game_state, &actions);

    if all_moves.is_empty() {
        return None;
    }
    let mut best_score = i32::MIN;
    let mut best_state = None;
    for mut next_state in all_moves {
        // The generated state represents one complete turn.
        next_state.player = game_state.player.opposite();
        let score = -negamax(&next_state, depth - 1, -MATE_SCORE, MATE_SCORE);

        if score > best_score {
            best_score = score;
            best_state = Some(next_state);
        }
    }

    best_state
}

fn negamax(game_state: &GameState, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return evaluate_board(game_state);
    }

    let actions = get_available_actions(game_state);
    let states = get_all_possible_states(game_state, &actions);

    if states.is_empty() {
        return -MATE_SCORE;
    }
    let mut best = i32::MIN;
    for mut next_state in states {
        next_state.player = game_state.player.opposite();

        let score = -negamax(&next_state, depth - 1, -beta, -alpha);

        best = best.max(score);
        alpha = alpha.max(score);

        if alpha >= beta {
            break;
        }
    }
    best
}

fn get_all_possible_states(game_state: &GameState, actions: &AvailableActions) -> Vec<GameState> {
    let has_capture = !actions.pawns_can_beat.is_empty() || !actions.dames_can_beat.is_empty();

    let mut states = Vec::new();
    if has_capture {
        // Every capture must be played.
        //
        // Each recursive sequence produces one complete
        // legal move.

        for (start, captures) in &actions.pawns_can_beat {
            for capture in captures {
                let next_state = beat(game_state.clone(), start.clone(), capture.clone());
                generate_capture_sequences(next_state, capture.1.clone(), &mut states);
            }
        }
        for (start, captures) in &actions.dames_can_beat {
            for capture in captures {
                let next_state = beat(game_state.clone(), start.clone(), capture.clone());
                generate_capture_sequences(next_state, capture.1.clone(), &mut states);
            }
        }

        return states;
    }
    for (start, destinations) in &actions.pawns_can_move {
        for dest in destinations {
            states.push(make_move(game_state.clone(), start.clone(), dest.clone()));
        }
    }
    for (start, destinations) in &actions.dames_can_move {
        for dest in destinations {
            states.push(make_move(game_state.clone(), start.clone(), dest.clone()));
        }
    }

    states
}

fn evaluate_board(game_state: &GameState) -> i32 {
    let my_color = match game_state.player {
        Player::Black => PawnColor::Black,
        Player::White => PawnColor::White,
    };

    let opponent_color = match game_state.player {
        Player::Black => PawnColor::White,
        Player::White => PawnColor::Black,
    };

    let mut score = 0;
    for field in game_state.board_state.values() {
        match (&field.pawn_color, &field.pawn_type) {
            (color, PawnType::Pawn) if *color == my_color => {
                score += 100;
            }

            (color, PawnType::Dame) if *color == my_color => {
                score += 300;
            }

            (color, PawnType::Pawn) if *color == opponent_color => {
                score -= 100;
            }

            (color, PawnType::Dame) if *color == opponent_color => {
                score -= 300;
            }

            _ => {}
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{get_start_board, FieldState, PawnColor, PawnType};
    use std::collections::HashMap;

    #[test]
    fn test_evaluation() {
        let mut board = HashMap::new();
        board.insert(
            "1_1".to_string(),
            FieldState {
                pawn_color: PawnColor::Black,
                pawn_type: PawnType::Pawn,
            },
        );
        board.insert(
            "8_8".to_string(),
            FieldState {
                pawn_color: PawnColor::White,
                pawn_type: PawnType::Pawn,
            },
        );

        let state = GameState {
            player: Player::Black,
            board_state: board,
        };
        assert_eq!(evaluate_board(&state), 0);
        assert_eq!(evaluate_board(&state), 0);
    }

    #[test]
    fn test_best_move_available() {
        let state = GameState {
            player: Player::Black,
            board_state: get_start_board(),
        };
        let best_state = get_best_move(&state, 2);
        assert!(best_state.is_some());
    }
}
