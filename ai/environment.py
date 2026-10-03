import gymnasium as gym
import numpy as np
from gymnasium import spaces

from api import CheckersApi
from encoding import decode_move, encode_move, encode_state


class CheckersEnv(gym.Env):

    def __init__(
        self,
        api_url: str,
        initial_state: dict,
    ):
        super().__init__()

        self.api = CheckersApi(api_url)
        self.agent_color = "Black"
        self.initial_state = {"player": self.agent_color, "board_state": initial_state}
        self.state = None

        self.action_space = spaces.Discrete(32 * 32)
        self.observation_space = spaces.Box(
            low=-2,
            high=2,
            shape=(33,),
            dtype=np.float32,
        )

        self.current_actions = []

    def make_move(self, source: str, destination: str):
        current_piece = self.state["board_state"][source]
        self.state["board_state"][destination] = current_piece
        self.state["board_state"][source] = {
            "pawn_color": "Empty",
            "pawn_type": "Empty",
        }
        self.state = self.api.make_move(self.state)

    def reset(self, seed=None, options=None):
        super().reset(seed=seed)
        self.state = self.initial_state.copy()
        self.current_actions = self.api.available_actions(self.state)

        observation = encode_state(self.state)
        return observation, {}

    def action_masks(self):
        mask = np.zeros(
            self.action_space.n,
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
        self.make_move(source=source, destination=destination)

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
        self.state = self.api.make_random_move(self.state)

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
