import requests


class CheckersApi:
    """
    A client for the Checkers API.
    """

    def __init__(self, base_url: str):
        """
        Initialize the Checkers API client.

        :param base_url: The base URL of the Checkers API.
        """
        self.base_url = base_url
        self.session = requests.Session()

    def available_actions(self, state: dict) -> list[dict]:
        """
        Get all available moves for the current state.

        :param state: The current board state and player.
        :return: A list of available moves.
        """
        response = self.session.post(
            f"{self.base_url}/get_available_moves",
            headers={"Content-Type": "application/json"},
            json=state,
        )

        response.raise_for_status()
        return response.json()

    def make_move(
        self,
        state: dict,
    ) -> dict | str:
        """
        Make an AI move based on the provided state.

        :param state: The current board state and player.
        :return: The new state dictionary or a string message if no moves are available.
        """
        response = self.session.post(
            f"{self.base_url}/make_ai_move",
            headers={"Content-Type": "application/json"},
            json=state,
        )

        response.raise_for_status()
        try:
            return response.json()
        except requests.exceptions.JSONDecodeError:
            return response.text

    def make_random_move(self, state: dict) -> dict | str:
        """
        Make a random move based on the provided state.

        :param state: The current board state and player.
        :return: The new state dictionary or a string message if no moves are available.
        """
        response = self.session.post(
            f"{self.base_url}/make_random_move",
            headers={"Content-Type": "application/json"},
            json=state,
        )

        response.raise_for_status()
        try:
            return response.json()
        except requests.exceptions.JSONDecodeError:
            return response.text
