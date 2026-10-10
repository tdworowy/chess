from typing import Any

import gymnasium as gym
import numpy as np
from gymnasium import spaces

from api import CheckersApi
from encoding import decode_move, encode_move, encode_state


class CheckersEnv(gym.Env):
    """
    Checkers environment for reinforcement learning using Gymnasium.
    """

    def __init__(
        self,
        api_url: str,
        initial_state: dict[str, dict[str, str]],
    ):
        """
        Initialize the Checkers environment.

        :param api_url: The URL of the Checkers API.
        :param initial_state: The initial board state dictionary.
        """
        super().__init__()

        self.api = CheckersApi(api_url)
        self.agent_color = "Black"
        self.initial_state = {"player": self.agent_color, "board_state": initial_state}
        self.state: dict[str, Any] | None = None

        self.action_space = spaces.Discrete(32 * 32)
        self.observation_space = spaces.Box(
            low=-2,
            high=2,
            shape=(33,),
            dtype=np.float32,
        )

        self.current_actions: list[dict[str, str]] = []

    def make_move(self, source: str, destination: str) -> dict[str, Any] | str:
        """
        Execute a move on the board and call the API to update the state.

        :param source: The source square identifier.
        :param destination: The destination square identifier.
        :return: The new state dictionary or a string message if no moves are available.
        :raises ValueError: If the move is invalid (e.g., source square is empty).
        """
        current_piece: dict | None = self.state["board_state"].get(source)

        if current_piece is None or current_piece["pawn_color"] == "Empty":
            raise ValueError(f"Cannot move from empty square: {source}")

        # Create a deep copy of the board state
        new_board_state = self.state["board_state"].copy()
        new_board_state[destination] = current_piece
        new_board_state[source] = {
            "pawn_color": "Empty",
            "pawn_type": "Empty",
        }

        next_player = "White" if self.agent_color == "Black" else "Black"
        agent_move_state = {"player": next_player, "board_state": new_board_state}

        response = self.api.make_move(agent_move_state)
        
        if isinstance(response, str) and "No available moves" in response:
            # Opponent has no moves, agent wins.
            # We keep the board as it was after agent's move.
            # But we must set the player to agent's color so that available_actions works for the next check.
            self.state = agent_move_state
            self.state["player"] = self.agent_color
            return response

        self.state = response
        # The backend does not toggle the player in the response.
        # After the opponent's move, it should be the agent's turn again.
        self.state["player"] = self.agent_color
        return response

    def reset(
        self,
        seed: int | None = None,
        options: dict[str, Any] | None = None,
    ) -> tuple[np.ndarray, dict[str, Any]]:
        """
        Reset the environment to its initial state.

        :param seed: Random seed for reproducibility.
        :param options: Additional options for reset.
        :return: A tuple of (observation, info).
        """
        super().reset(seed=seed)
        self.state = self.initial_state.copy()
        self.current_actions = self.api.available_actions(self.state)

        observation = encode_state(self.state)
        return observation, {}

    def action_masks(self) -> np.ndarray:
        """
        Generate a mask of valid actions for the current state.

        :return: A boolean array where True indicates a valid action.
        """
        mask = np.zeros(
            32 * 32,
            dtype=bool,
        )

        for move in self.current_actions:
            action = encode_move(
                move["from"],
                move["to"],
            )

            mask[action] = True
        return mask

    def step(
        self,
        action: int,
    ) -> tuple[np.ndarray, float, bool, bool, dict[str, Any]]:
        """
        Execute a single step in the environment.

        This includes the agent's move and the opponent's move.

        :param action: The action integer to execute.
        :return: A tuple of (observation, reward, terminated, truncated, info).
        """
        source, destination = decode_move(int(action))

        # Defensive check.
        mask = self.action_masks()
        if not mask[action]:
            return (
                encode_state(self.state),
                -1.0,
                True,
                False,
                {"invalid_action": True},
            )

        # Agent move + Opponent move
        res = self.make_move(source=source, destination=destination)

        # Check if the opponent had no moves (Agent wins)
        if isinstance(res, str) and "No available moves" in res:
            return (
                encode_state(self.state),
                1.0,
                True,
                False,
                {"winner": self.agent_color, "reason": "Opponent has no moves"},
            )

        # Check if we (Agent) have any moves left for the next turn
        self.current_actions = self.api.available_actions(self.state)
        if not self.current_actions:
            return (
                encode_state(self.state),
                -1.0,
                True,
                False,
                {"winner": "White" if self.agent_color == "Black" else "Black"},
            )

        return (
            encode_state(self.state),
            0.0,
            False,
            False,
            {},
        )
