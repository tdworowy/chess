import json

import pytest
import requests


@pytest.mark.parametrize(
    "url,result",
    [
        ("http://localhost:8080/healthcheck", {"message": "OK"}),
        (
            "http://localhost:8080/get_example",
            {
                "player": "Black",
                "board_state": {"1_1": {"pawn_color": "Black", "pawn_type": "Pawn"}},
            },
        ),
    ],
)
def test_smoke(url: str, result: dict):
    assert json.loads(requests.get(url).json()) == result


def test_make_move():
    data = {
        "player": "Black",
        "board_state": {"1_1": {"pawn_color": "Black", "pawn_type": "Pawn"}},
    }
    response = requests.post(
        "http://localhost:8080/make_move",
        headers={"Content-Type": "application/json"},
        json=data,
    )
    assert response.status_code == 200
    assert json.loads(response.json()) == data
