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
        initial_state: dict,
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

        self.current_actions: list[dict] = []

    def make_move(self, source: str, destination: str):
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

        # Create a deep copy of the board state to avoid modifying it in place if make_move fails
        new_board_state = self.state["board_state"].copy()
        new_board_state[destination] = current_piece
        new_board_state[source] = {
            "pawn_color": "Empty",
            "pawn_type": "Empty",
        }

        new_state = {"player": self.state["player"], "board_state": new_board_state}

        response = self.api.make_move(new_state)
        if isinstance(response, str) and "No available moves" in response:
            # If the move leads to no more moves for anyone (should not happen in checkers unless game ends)
            # or if the API just returns this when a player has no moves.
            # In our case, make_move is called after an agent move to get the state after AI (opponent) move.
            self.state = new_state
            return response

        self.state = response
        return response

    def reset(self, seed=None, options=None):
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

    def action_masks(self):
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

    def step(self, action):
        """
        Execute a single step in the environment.

        This includes the agent's move and the opponent's (random) move.

        :param action: The action integer to execute.
        :return: A tuple of (observation, reward, terminated, truncated, info).
        """
        source, destination = decode_move(int(action))

        # Defensive check.
        #
        # During actual MaskablePPO training this should
        # never happen.
        mask = self.action_masks()
        if not mask[action]:
            return (
                encode_state(self.state),
                -1.0,
                True,
                False,
                {"invalid_action": True},
            )

        # Agent move
        res = self.make_move(source=source, destination=destination)

        # Check if the move resulted in "No available moves" from the API
        if isinstance(res, str) and "No available moves" in res:
            return (
                encode_state(self.state),
                1.0,
                True,
                False,
                {"winner": self.agent_color, "reason": "No available moves"},
            )

        # Check whether opponent has lost
        opponent_actions = self.api.available_actions(self.state)
        if not opponent_actions:
            return (
                encode_state(self.state),
                1.0,
                True,
                False,
                {"winner": self.agent_color},
            )
        # Opponent move
        response = self.api.make_random_move(self.state)
        if isinstance(response, str) and "No available moves" in response:
            return (
                encode_state(self.state),
                1.0,
                True,
                False,
                {"winner": self.agent_color, "reason": "Opponent has no moves"},
            )
        self.state = response

        # Check whether we lost
        self.current_actions = self.api.available_actions(self.state)
        if not self.current_actions:
            return (
                encode_state(self.state),
                -1.0,
                True,
                False,
                {"winner": "White"},
            )
        return (
            encode_state(self.state),
            0.0,
            False,
            False,
            {},
        )
