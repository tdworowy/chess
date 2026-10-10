from typing import Any


def generate_init_board() -> dict[str, dict[str, str]]:
    """
    Generate an initial checkers board with all squares (including empty ones).

    :return: A dictionary mapping square coordinates (e.g., "1_2") to piece info.
    """
    result = {}
    for i in range(1, 9):
        for j in range(1, 9):
            if i == 2 and j % 2 != 0:
                result[f"{i}_{j}"] = {"pawn_color": "Black", "pawn_type": "Pawn"}
                continue
            if (i == 1 or i == 3) and j % 2 == 0:
                result[f"{i}_{j}"] = {"pawn_color": "Black", "pawn_type": "Pawn"}
                continue
            if i == 7 and j % 2 == 0:
                result[f"{i}_{j}"] = {"pawn_color": "White", "pawn_type": "Pawn"}
                continue
            if (i == 6 or i == 8) and j % 2 != 0:
                result[f"{i}_{j}"] = {"pawn_color": "White", "pawn_type": "Pawn"}
                continue
            result[f"{i}_{j}"] = {"pawn_color": "Empty", "pawn_type": "Empty"}
    return result


def generate_init_board_no_empty() -> dict[str, dict[str, str]]:
    """
    Generate an initial checkers board excluding empty squares.

    :return: A dictionary mapping occupied square coordinates to piece info.
    """
    init_board = generate_init_board()
    return {
        position: pawn
        for position, pawn in init_board.items()
        if pawn["pawn_color"] != "Empty"
    }
