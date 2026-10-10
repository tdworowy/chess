import numpy as np
from typing import Any


def encode_action(src: int, dst: int) -> int:
    """
    Encode a move from source index and destination index into a single integer action.

    :param src: Source square index (0-31).
    :param dst: Destination square index (0-31).
    :return: Encoded action.
    """
    return src * 32 + dst


def decode_action(action: int) -> tuple[int, int]:
    """
    Decode a single integer action into source index and destination index.

    :param action: Encoded action integer.
    :return: A tuple of (source_index, destination_index).
    """
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


def square_index(square: str) -> int:
    """
    Convert a square string (e.g., "1_2" or "1") to its 0-31 index.

    :param square: Square identifier string.
    :return: Index of the square (0-31).
    :raises ValueError: If the square number is out of the valid range [1, 32].
    """
    if "_" in square:
        return SQUARE_TO_INDEX[square]

    number = int(square)
    if not 1 <= number <= 32:
        raise ValueError(f"Invalid checkers square: {square}")

    return number - 1


def encode_move(source: str, destination: str) -> int:
    """
    Encode a move from source square string and destination square string into a single integer action.

    :param source: Source square string identifier.
    :param destination: Destination square string identifier.
    :return: Encoded action integer.
    """
    source_index = square_index(source)
    destination_index = square_index(destination)

    return source_index * 32 + destination_index


def decode_move(action: int) -> tuple[str, str]:
    """
    Decode an action integer into source and destination square strings.

    :param action: Encoded action integer.
    :return: A tuple of (source_square_string, destination_square_string).
    """
    source, destination = divmod(action, 32)

    return (
        INDEX_TO_SQUARE[source],
        INDEX_TO_SQUARE[destination],
    )


def encode_state(state: dict[str, Any]) -> np.ndarray:
    """
    Encode the board state into a numerical format suitable for RL.

    The encoding includes piece positions (1 for Black Pawn, 2 for Black King,
    -1 for White Pawn, -2 for White King) and the current player.

    :param state: The current game state dictionary.
    :return: A numpy array representing the encoded state.
    """
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
