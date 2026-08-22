import json

import pytest
import requests

from utils import generate_init_board, generate_init_board_no_empty


@pytest.mark.parametrize(
    "url,result",
    [
        ("http://localhost:8080/healthcheck", {"message": "OK"}),
        (
            "http://localhost:8080/get_example",
            {
                "board_state": generate_init_board_no_empty(),
                "player": "Black",
            },
        ),
    ],
)
def test_smoke(url: str, result: dict):
    response = requests.get(url).json()
    print(f"response: {response}")
    print(f"result: {result}")
    assert response == result


def test_make_move():
    # TODO better assertion(?)
    data = {
        "board_state": generate_init_board(),
        "player": "Black",
    }
    response = requests.post(
        "http://localhost:8080/make_ai_move",
        headers={"Content-Type": "application/json"},
        json=data,
    )
    assert response.status_code == 200
    assert response.json() != data


def test_make_random_move():
    data = {
        "player": "Black",
        "board_state": generate_init_board(),
    }
    response = requests.post(
        "http://localhost:8080/make_random_move",
        headers={"Content-Type": "application/json"},
        json=data,
    )
    assert response.status_code == 200
    assert response.json() != data
