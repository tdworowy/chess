import json

import pytest
import requests


def generate_init_board() -> dict:
    result = {}
    for i in range(1, 9):
        for j in range(1, 9):
            print(f"{i}_{j}")
            if (i == 1 or i == 2) and j % 2 == 0:
                result[f"{i}_{j}"] = {"pawn_color": "Black", "pawn_type": "Pawn"}
                continue
            if i == 3 and j % 2 != 0:
                result[f"{i}_{j}"] = {"pawn_color": "Black", "pawn_type": "Pawn"}
                continue

            if (i == 6 or i == 7) and j % 2 == 0:
                result[f"{i}_{j}"] = {"pawn_color": "White", "pawn_type": "Pawn"}
                continue
            if i == 8 and j % 2 != 0:
                result[f"{i}_{j}"] = {"pawn_color": "White", "pawn_type": "Pawn"}
                continue

            result[f"{i}_{j}"] = {"pawn_color": "Empty", "pawn_type": "Empty"}
    return result


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
    print(json.loads(requests.get(url).json()))
   # assert json.loads(requests.get(url).json()) == result


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
