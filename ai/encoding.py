import numpy as np


def encode_action(src: int, dst: int) -> int:
    return src * 32 + dst


def decode_action(action: int) -> tuple[int, int]:
    return divmod(action, 32)


PLAYABLE_SQUARES = [
    f"{row}_{col}" for row in range(1, 9) for col in range(1, 9) if (row + col) % 2 == 1
]

SQUARE_TO_INDEX = {square: i for i, square in enumerate(PLAYABLE_SQUARES)}
INDEX_TO_SQUARE = {i: square for square, i in SQUARE_TO_INDEX.items()}


def encode_move(source: str, destination: str) -> int:
    return SQUARE_TO_INDEX[source] * 32 + SQUARE_TO_INDEX[destination]


def decode_move(action: int) -> tuple[str, str]:
    source, destination = divmod(action, 32)

    return (
        INDEX_TO_SQUARE[source],
        INDEX_TO_SQUARE[destination],
    )


def encode_state(state: dict) -> np.ndarray:
    result = np.zeros(33, dtype=np.float32)

    values = {
        ("Black", "Pawn"): 1,
        ("Black", "Dame"): 2,
        ("White", "Pawn"): -1,
        ("White", "Dame"): -2,
        ("Empty", "Empty"): 0,
    }

    board = state["board_state"]

    for square, i in SQUARE_TO_INDEX.items():
        square_sate = board[square]
        result[i] = values[(square_sate["pawn_color"], square_sate["pawn_type"])]

    result[32] = 1 if state["player"] == "Black" else -1

    return result
