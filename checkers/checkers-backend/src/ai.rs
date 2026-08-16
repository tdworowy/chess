use crate::game::{
    beat, get_available_actions, make_move, AvailableActions, GameState, PawnColor, PawnType,
    Player,
};

pub fn get_best_move(
    game_state: &GameState,
    available_actions: &AvailableActions,
    depth: i32,
) -> Option<GameState> {
    let player = game_state.player.clone();
    let mut best_score = f32::NEG_INFINITY;
    let mut best_state = None;

    let all_moves = get_all_possible_states(game_state, available_actions);

    if all_moves.is_empty() {
        return None;
    }

    for mut next_state in all_moves {
        next_state.player = player.opposite();
        let score = -minimax(
            &next_state,
            depth - 1,
            f32::NEG_INFINITY,
            f32::INFINITY,
            false,
        );
        if score > best_score {
            best_score = score;
            best_state = Some(next_state);
        }
    }

    best_state
}

fn minimax(
    game_state: &GameState,
    depth: i32,
    mut alpha: f32,
    mut beta: f32,
    maximizing_player: bool,
) -> f32 {
    if depth == 0 {
        return evaluate_board(game_state, &game_state.player);
    }

    let available_actions = get_available_actions(game_state);
    let all_moves = get_all_possible_states(game_state, &available_actions);

    if all_moves.is_empty() {
        // If no moves, current player loses
        return if maximizing_player { -100.0 } else { 100.0 };
    }

    if maximizing_player {
        let mut max_eval = f32::NEG_INFINITY;
        for mut next_state in all_moves {
            next_state.player = game_state.player.opposite();
            let eval = minimax(&next_state, depth - 1, alpha, beta, false);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = f32::INFINITY;
        for mut next_state in all_moves {
            next_state.player = game_state.player.opposite();
            let eval = minimax(&next_state, depth - 1, alpha, beta, true);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

fn get_all_possible_states(game_state: &GameState, actions: &AvailableActions) -> Vec<GameState> {
    let mut states = Vec::new();

    for (start, destinations) in &actions.pawns_can_move {
        for dest in destinations {
            states.push(make_move(game_state.clone(), start.clone(), dest.clone()));
        }
    }
    for (start, destinations) in &actions.pawns_can_beat {
        for dest in destinations {
            states.push(beat(game_state.clone(), start.clone(), dest.clone()));
        }
    }
    for (start, destinations) in &actions.dames_can_move {
        for dest in destinations {
            states.push(make_move(game_state.clone(), start.clone(), dest.clone()));
        }
    }
    for (start, destinations) in &actions.dames_can_beat {
        for dest in destinations {
            states.push(beat(game_state.clone(), start.clone(), dest.clone()));
        }
    }

    states
}

fn evaluate_board(game_state: &GameState, player: &Player) -> f32 {
    let mut score = 0.0;
    let (my_color, opp_color) = match player {
        Player::Black => (PawnColor::Black, PawnColor::White),
        Player::White => (PawnColor::White, PawnColor::Black),
    };

    for field in game_state.board_state.values() {
        if field.pawn_color == my_color {
            match field.pawn_type {
                PawnType::Pawn => score += 1.0,
                PawnType::Dame => score += 3.0,
                _ => {}
            }
        } else if field.pawn_color == opp_color {
            match field.pawn_type {
                PawnType::Pawn => score -= 1.0,
                PawnType::Dame => score -= 3.0,
                _ => {}
            }
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
        assert_eq!(evaluate_board(&state, &Player::Black), 0.0);
        assert_eq!(evaluate_board(&state, &Player::White), 0.0);
    }

    #[test]
    fn test_best_move_available() {
        let state = GameState {
            player: Player::Black,
            board_state: get_start_board(),
        };
        let actions = get_available_actions(&state);
        let best_state = get_best_move(&state, &actions, 2);
        assert!(best_state.is_some());
    }
}
