import numpy as np
import pytest

from encoding import (SQUARE_TO_INDEX, decode_action, decode_move,
                      encode_action, encode_move, encode_state, square_index)


def test_action_encoding_decoding():
    for src in range(32):
        for dst in range(32):
            action = encode_action(src, dst)
            assert action == src * 32 + dst
            s, d = decode_action(action)
            assert s == src
            assert d == dst


def test_square_index():
    assert square_index("1_2") == SQUARE_TO_INDEX["1_2"]
    assert square_index("1") == 0
    assert square_index("32") == 31

    with pytest.raises(ValueError):
        square_index("0")
    with pytest.raises(ValueError):
        square_index("33")
    with pytest.raises(KeyError):
        square_index("1_1")  # Not a playable square


def test_move_encoding_decoding():
    src_sq, dst_sq = list(SQUARE_TO_INDEX.keys())[0], list(SQUARE_TO_INDEX.keys())[1]
    src_idx, dst_idx = SQUARE_TO_INDEX[src_sq], SQUARE_TO_INDEX[dst_sq]

    action = encode_move(src_sq, dst_sq)
    assert action == src_idx * 32 + dst_idx

    src, dst = decode_move(action)
    assert src == src_sq
    assert dst == dst_sq


def test_encode_state():
    state = {
        "player": "Black",
        "board_state": {
            "1_2": {"pawn_color": "Black", "pawn_type": "Pawn"},
            "8_7": {"pawn_color": "White", "pawn_type": "King"},
        },
    }
    encoded = encode_state(state)
    assert encoded.shape == (33,)
    assert encoded[SQUARE_TO_INDEX["1_2"]] == 1.0
    assert encoded[SQUARE_TO_INDEX["8_7"]] == -2.0
    assert encoded[32] == 1.0  # Black player

    # Check empty squares are 0
    for sq, idx in SQUARE_TO_INDEX.items():
        if sq not in ["1_2", "8_7"]:
            assert encoded[idx] == 0.0

    state["player"] = "White"
    encoded = encode_state(state)
    assert encoded[32] == -1.0
