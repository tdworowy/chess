from unittest.mock import patch

import pytest
import requests

from src.api import CheckersApi


def test_api_init():
    api = CheckersApi("http://localhost:8080")
    assert api.base_url == "http://localhost:8080"
    assert isinstance(api.session, requests.Session)


def test_available_actions():
    api = CheckersApi("http://localhost:8080")
    state = {"player": "Black", "board_state": {}}
    mock_response = [{"from": "1_2", "to": "2_3"}]

    with patch.object(api.session, "post") as mock_post:
        mock_post.return_value.status_code = 200
        mock_post.return_value.json.return_value = mock_response

        actions = api.available_actions(state)

        assert actions == mock_response
        mock_post.assert_called_once_with(
            "http://localhost:8080/get_available_moves",
            headers={"Content-Type": "application/json"},
            json=state,
        )


def test_make_move_json():
    api = CheckersApi("http://localhost:8080")
    state = {"player": "Black", "board_state": {}}
    mock_response = {
        "player": "White",
        "board_state": {"2_3": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }

    with patch.object(api.session, "post") as mock_post:
        mock_post.return_value.status_code = 200
        mock_post.return_value.json.return_value = mock_response

        result = api.make_move(state)

        assert result == mock_response


def test_make_move_text():
    api = CheckersApi("http://localhost:8080")
    state = {"player": "Black", "board_state": {}}
    mock_text = "No available moves"

    with patch.object(api.session, "post") as mock_post:
        mock_post.return_value.status_code = 200
        mock_post.return_value.json.side_effect = requests.exceptions.JSONDecodeError(
            "msg", "doc", 0
        )
        mock_post.return_value.text = mock_text

        result = api.make_move(state)

        assert result == mock_text


def test_make_random_move_json():
    api = CheckersApi("http://localhost:8080")
    state = {"player": "Black", "board_state": {}}
    mock_response = {"player": "White", "board_state": {}}

    with patch.object(api.session, "post") as mock_post:
        mock_post.return_value.status_code = 200
        mock_post.return_value.json.return_value = mock_response

        result = api.make_random_move(state)

        assert result == mock_response


def test_api_error_handling():
    api = CheckersApi("http://localhost:8080")
    state = {"player": "Black", "board_state": {}}

    with patch.object(api.session, "post") as mock_post:
        mock_post.return_value.status_code = 500
        mock_post.return_value.raise_for_status.side_effect = (
            requests.exceptions.HTTPError("Error")
        )

        with pytest.raises(requests.exceptions.HTTPError):
            api.available_actions(state)
