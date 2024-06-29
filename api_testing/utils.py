def generate_init_board() -> dict:
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


if __name__ == "__main__":
    print(generate_init_board())
