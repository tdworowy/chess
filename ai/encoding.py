import numpy as np


def encode_action(src: int, dst: int) -> int:
    return src * 32 + dst


def decode_action(action: int) -> tuple[int, int]:
    return divmod(action, 32)


PLAYABLE_SQUARES = [
    f"{row}_{col}" for row in range(1, 9) for col in range(1, 9) if (row + col) % 2 == 1
]

SQUARE_TO_INDEX = {
    f"{row}_{col}": index
    for index, (row, col) in enumerate(
        (row, col) for row in range(1, 9) for col in range(1, 9) if (row + col) % 2 == 1
    )
}

INDEX_TO_SQUARE = {index: square for square, index in SQUARE_TO_INDEX.items()}


# def encode_move(source: str, destination: str) -> int:
#     source_index = int(source) - 1
#     destination_index = int(destination) - 1
#
#     return source_index * 32 + destination_index


def square_index(square: str) -> int:
    if "_" in square:
        return SQUARE_TO_INDEX[square]

    number = int(square)
    if not 1 <= number <= 32:
        raise ValueError(f"Invalid checkers square: {square}")

    return number - 1


def encode_move(source: str, destination: str) -> int:
    source_index = square_index(source)
    destination_index = square_index(destination)

    return source_index * 32 + destination_index


def decode_move(action: int) -> tuple[str, str]:
    source, destination = divmod(action, 32)

    return (
        INDEX_TO_SQUARE[source],
        INDEX_TO_SQUARE[destination],
    )


def encode_state(state: dict) -> np.ndarray:
    board = state["board_state"]
    encoded = np.zeros(33, dtype=np.float32)

    for square, index in SQUARE_TO_INDEX.items():
        field = board.get(square)
        # Missing square = empty square
        if field is None:
            continue

        color = field["pawn_color"]
        piece = field["pawn_type"]

        if color == "Black":
            encoded[index] = 1 if piece == "Pawn" else 2
        elif color == "White":
            encoded[index] = -1 if piece == "Pawn" else -2

    encoded[32] = 1 if state["player"] == "Black" else -1

    return encoded
