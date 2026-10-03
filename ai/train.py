"""
Script to train a MaskablePPO agent on the Checkers environment.
"""

from sb3_contrib import MaskablePPO

from environment import CheckersEnv
from utils import generate_init_board_no_empty

env = CheckersEnv(
    api_url="http://localhost:8080",
    initial_state=generate_init_board_no_empty(),
)

model = MaskablePPO(
    "MlpPolicy",
    env,
    verbose=1,
    tensorboard_log="./runs/",
    n_steps=128,
    batch_size=64,  # small numbers for testing
    policy_kwargs={
        "net_arch": {
            "pi": [256, 256],
            "vf": [256, 256],
        }
    },
)

model.learn(
    total_timesteps=500,  # 1_000_000 TODO for more iteration it need better performance
    progress_bar=True,
)

model.save("checkers_ppo")
