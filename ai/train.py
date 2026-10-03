# train.py

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
    policy_kwargs={
        "net_arch": {
            "pi": [256, 256],
            "vf": [256, 256],
        }
    },
)

model.learn(
    total_timesteps=1_000_0,  # 1_000_000
    progress_bar=True,
)

model.save("checkers_ppo")
