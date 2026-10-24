from unittest.mock import MagicMock

import numpy as np
import pytest

from src.encoding import encode_move
from src.environment import CheckersEnv


@pytest.fixture
def env():
    initial_state = {"1_2": {"pawn_color": "Black", "pawn_type": "Pawn"}}
    env = CheckersEnv(api_url="http://mock", initial_state=initial_state)
    env.api = MagicMock()
    return env


def test_env_init(env):
    assert env.agent_color == "Black"
    assert env.action_space.n == 32 * 32
    assert env.observation_space.shape == (33,)


def test_env_reset(env):
    env.api.available_actions.return_value = [{"from": "1_2", "to": "2_3"}]
    obs, info = env.reset()

    assert obs.shape == (33,)
    assert env.state["player"] == "Black"
    assert env.current_actions == [{"from": "1_2", "to": "2_3"}]
    env.api.available_actions.assert_called_with(env.state)


def test_action_masks(env):
    env.current_actions = [{"from": "1_2", "to": "2_3"}]
    mask = env.action_masks()

    action = encode_move("1_2", "2_3")
    assert mask[action] == True
    assert np.sum(mask) == 1


def test_make_move_success(env):
    env.state = {
        "player": "Black",
        "board_state": {"1_2": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    mock_response = {
        "player": "White",
        "board_state": {"2_3": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    env.api.apply_move.return_value = {
        "player": "Black",
        "board_state": {"2_3": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    env.api.make_move.return_value = mock_response

    res = env.make_move("1_2", "2_3")

    assert res == mock_response
    assert env.state == mock_response
    env.api.apply_move.assert_called_once_with(env.initial_state, "1_2", "2_3")
    assert env.api.make_move.call_args.args[0]["player"] == "White"


def test_make_move_invalid_is_rejected_by_api(env):
    env.state = {"player": "Black", "board_state": {}}
    env.api.apply_move.side_effect = ValueError("Illegal move")
    with pytest.raises(ValueError, match="Illegal move"):
        env.make_move("1_2", "2_3")
    env.api.make_move.assert_not_called()


def test_step_invalid_action(env):
    env.api.available_actions.return_value = []
    env.reset()

    # Action for 1_2 to 2_3 when it's not in current_actions
    action = encode_move("1_2", "2_3")
    obs, reward, terminated, truncated, info = env.step(action)

    assert reward == -1.0
    assert terminated == True
    assert info["invalid_action"] == True


def test_step_rewards_material_gain(env):
    env.initial_state = {
        "player": "Black",
        "board_state": {
            "3_2": {"pawn_color": "Black", "pawn_type": "Pawn"},
            "4_3": {"pawn_color": "White", "pawn_type": "Pawn"},
        },
    }
    capture = {"from": "3_2", "to": "5_4"}
    env.api.available_actions.side_effect = [
        [capture],
        [{"from": "5_4", "to": "6_3"}],
    ]
    env.api.apply_move.return_value = {
        "player": "Black",
        "board_state": {"5_4": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    env.api.make_move.return_value = {
        "player": "Black",
        "board_state": {"5_4": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    env.reset()

    action = encode_move("3_2", "5_4")
    _, reward, terminated, _, info = env.step(action)

    assert reward == pytest.approx(0.1)
    assert not terminated
    assert info["material_delta"] == 1


def test_step_win_on_no_moves(env):
    env.api.available_actions.return_value = [{"from": "1_2", "to": "2_3"}]
    env.reset()

    action = encode_move("1_2", "2_3")
    env.api.apply_move.return_value = {
        "player": "Black",
        "board_state": {"2_3": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    env.api.make_move.return_value = "No available moves"

    obs, reward, terminated, truncated, info = env.step(action)

    assert reward == 1.0
    assert terminated == True
    assert info["winner"] == "Black"


def test_step_win_when_ai_has_no_moves(env):
    env.api.available_actions.side_effect = [
        [{"from": "1_2", "to": "2_3"}],  # for reset
    ]
    env.reset()

    action = encode_move("1_2", "2_3")
    env.api.apply_move.return_value = {
        "player": "Black",
        "board_state": {"2_3": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    env.api.make_move.return_value = "No available moves"

    obs, reward, terminated, truncated, info = env.step(action)

    assert reward == 1.0
    assert terminated == True
    assert info["reason"] == "Opponent has no moves"


def test_step_loss(env):
    env.api.available_actions.side_effect = [
        [{"from": "1_2", "to": "2_3"}],  # for reset
        [],  # for our actions check - we have no moves!
    ]
    env.reset()

    action = encode_move("1_2", "2_3")
    env.api.apply_move.return_value = {
        "player": "Black",
        "board_state": {"2_3": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    env.api.make_move.return_value = {
        "player": "White",
        "board_state": {"2_3": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    obs, reward, terminated, truncated, info = env.step(action)

    assert reward == -1.0
    assert terminated == True
    assert info["winner"] == "White"
