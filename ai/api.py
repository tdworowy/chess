import requests


class CheckersApi:
    def __init__(self, base_url: str):
        self.base_url = base_url
        self.session = requests.Session()

    def available_actions(self, state: dict) -> list[dict]:
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
    ) -> dict:
        response = self.session.post(
            f"{self.base_url}/make_ai_move",
            headers={"Content-Type": "application/json"},
            json=state,
        )

        response.raise_for_status()
        return response.json()

    def make_random_move(self, state: dict) -> dict:
        response = self.session.post(
            f"{self.base_url}/make_random_move",
            headers={"Content-Type": "application/json"},
            json=state,
        )

        response.raise_for_status()
        return response.json()
