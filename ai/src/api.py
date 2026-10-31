import requests
from typing import Any


class CheckersApi:
    """
    A client for the Checkers API.
    """

    def __init__(self, base_url: str, timeout: float | None = None):
        """
        Initialize the Checkers API client.

        :param base_url: The base URL of the Checkers API.
        :param timeout: Maximum number of seconds to wait for an API response.
        """
        self.base_url = base_url
        self.timeout = timeout
        self.session = requests.Session()

    def _post(self, url: str, state: dict[str, Any]) -> requests.Response:
        request_options: dict[str, Any] = {
            "headers": {"Content-Type": "application/json"},
            "json": state,
        }
        if self.timeout is not None:
            request_options["timeout"] = self.timeout
        return self.session.post(url, **request_options)

    def available_actions(self, state: dict[str, Any]) -> list[dict[str, str]]:
        """
        Get all available moves for the current state.

        :param state: The current board state and player.
        :return: A list of available moves.
        """
        response = self._post(f"{self.base_url}/get_available_moves", state)

        response.raise_for_status()
        return response.json()

    def apply_move(
        self, state: dict[str, Any], source: str, destination: str
    ) -> dict[str, Any]:
        """Validate and apply a player's move using the pre-move state."""
        response = self._post(
            f"{self.base_url}/apply_move",
            {"state": state, "from": source, "to": destination},
        )
        response.raise_for_status()
        return response.json()

    def make_move(
        self,
        state: dict[str, Any],
    ) -> dict[str, Any] | str:
        """
        Make an AI move based on the provided state.

        :param state: The current board state and player.
        :return: The new state dictionary or a string message if no moves are available.
        """
        response = self._post(f"{self.base_url}/make_ai_move_easy", state)

        response.raise_for_status()
        try:
            return response.json()
        except requests.exceptions.JSONDecodeError:
            return response.text
