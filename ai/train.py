"""
Script to train a MaskablePPO agent on the Checkers environment.
"""

from sb3_contrib import MaskablePPO
from stable_baselines3.common.callbacks import EvalCallback

from environment import CheckersEnv
from utils import generate_init_board_no_empty

# Create evaluation environment
eval_env = CheckersEnv(
    api_url="http://localhost:8080",
    initial_state=generate_init_board_no_empty(),
)

env = CheckersEnv(
    api_url="http://localhost:8080",
    initial_state=generate_init_board_no_empty(),
)

# Use EvalCallback for periodic evaluation and logging
eval_callback = EvalCallback(
    eval_env,
    best_model_save_path="./logs/",
    log_path="./logs/",
    eval_freq=1000,
    deterministic=True,
    render=False,
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
    total_timesteps=5000,  # Increased for better visualization
    progress_bar=True,
    callback=eval_callback,
)

model.save("checkers_ppo")
