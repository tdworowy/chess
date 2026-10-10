from src.utils import generate_init_board, generate_init_board_no_empty


def test_generate_init_board():
    board = generate_init_board()
    assert len(board) == 64
    assert "1_2" in board
    assert board["1_2"] == {"pawn_color": "Black", "pawn_type": "Pawn"}
    assert "7_2" in board
    assert board["7_2"] == {"pawn_color": "White", "pawn_type": "Pawn"}
    assert "4_4" in board
    assert board["4_4"] == {"pawn_color": "Empty", "pawn_type": "Empty"}


def test_generate_init_board_no_empty():
    board = generate_init_board_no_empty()
    assert len(board) == 24  # 12 black, 12 white
    for pos, piece in board.items():
        assert piece["pawn_color"] != "Empty"
        assert piece["pawn_type"] != "Empty"
