import json

import pytest
import requests

from utils import generate_init_board


@pytest.mark.parametrize(
    "url,result",
    [
        ("http://localhost:8080/healthcheck", {"message": "OK"}),
        (
            "http://localhost:8080/get_example",
            {
                "player": "Black",
                "board_state": generate_init_board(),
            },
        ),
    ],
)
def test_smoke(url: str, result: dict):
    response = json.loads(requests.get(url).json())
    print(response)
    assert response == result


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
