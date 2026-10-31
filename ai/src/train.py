"""Train a MaskablePPO agent and report whether its checkers play is improving."""

from datetime import datetime, timezone
import os
from pathlib import Path

from gymnasium.wrappers import TimeLimit
from sb3_contrib import MaskablePPO
from stable_baselines3.common.monitor import Monitor

from environment import CheckersEnv
from training_callbacks import EvaluationDiagnosticsCallback, TrainingEpisodeCallback
from utils import generate_init_board_no_empty


AI_ROOT = Path(__file__).resolve().parents[1]
LOG_DIR = AI_ROOT / "logs"
RUNS_DIR = AI_ROOT / "runs"
API_URL = os.environ.get("CHECKERS_API_URL", "http://localhost:8080")
TOTAL_TIMESTEPS = int(os.environ.get("CHECKERS_TRAINING_TIMESTEPS", "200000"))
MAX_EPISODE_STEPS = 200
EVAL_FREQUENCY = 5_000
EVAL_EPISODES = 10

# 2,048 steps with 128-sample minibatches gives PPO 16 minibatches per epoch.
N_STEPS = 2_048
BATCH_SIZE = 128
N_EPOCHS = 10
LEARNING_RATE = 3e-4

# Checkers games can run for many turns, so retain more of the terminal
# win/loss signal and keep a small amount of policy exploration.
GAMMA = 0.995
GAE_LAMBDA = 0.98
ENTROPY_COEFFICIENT = 0.005
TARGET_KL = 0.03


def make_env() -> Monitor:
    """Create an episode-monitored environment with a limit for endless games."""
    env = CheckersEnv(
        api_url=API_URL,
        initial_state=generate_init_board_no_empty(),
    )
    return Monitor(TimeLimit(env, max_episode_steps=MAX_EPISODE_STEPS))


def main() -> None:
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    RUNS_DIR.mkdir(parents=True, exist_ok=True)
    run_id = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")

    print(f"Training run: {run_id}")
    print(f"Checkers API: {API_URL}")
    print(f"Timesteps: {TOTAL_TIMESTEPS:,}; evaluation every {EVAL_FREQUENCY:,} steps")
    print(
        f"PPO: n_steps={N_STEPS}, batch_size={BATCH_SIZE}, n_epochs={N_EPOCHS}, "
        f"gamma={GAMMA}, gae_lambda={GAE_LAMBDA}, ent_coef={ENTROPY_COEFFICIENT}"
    )
    print(f"TensorBoard logs: {RUNS_DIR}")
    print(f"Episode/evaluation reports: {LOG_DIR}")

    training_env = make_env()
    evaluation_env = make_env()

    training_callback = TrainingEpisodeCallback(
        report_path=LOG_DIR / "training_episodes.csv",
        run_id=run_id,
        agent_color="Black",
        report_every_episodes=25,
    )
    evaluation_callback = EvaluationDiagnosticsCallback(
        evaluation_env,
        report_path=LOG_DIR / "evaluation_history.csv",
        run_id=run_id,
        agent_color="Black",
        best_model_save_path=str(LOG_DIR),
        log_path=str(LOG_DIR),
        eval_freq=EVAL_FREQUENCY,
        n_eval_episodes=EVAL_EPISODES,
        deterministic=True,
        render=False,
        verbose=0,
    )

    model = MaskablePPO(
        "MlpPolicy",
        training_env,
        verbose=1,
        tensorboard_log=str(RUNS_DIR),
        learning_rate=LEARNING_RATE,
        n_steps=N_STEPS,
        batch_size=BATCH_SIZE,
        n_epochs=N_EPOCHS,
        gamma=GAMMA,
        gae_lambda=GAE_LAMBDA,
        ent_coef=ENTROPY_COEFFICIENT,
        target_kl=TARGET_KL,
        policy_kwargs={
            "net_arch": {
                "pi": [256, 256],
                "vf": [256, 256],
            }
        },
    )

    model.learn(
        total_timesteps=TOTAL_TIMESTEPS,
        progress_bar=True,
        callback=[training_callback, evaluation_callback],
    )

    model_path = AI_ROOT / "checkers_ppo"
    model.save(str(model_path))
    print(f"Training complete. Model saved to {model_path.with_suffix('.zip')}")
    print("Open TensorBoard with: uv run python src/launch_tensorboard.py (from ai/)")

    training_env.close()
    evaluation_env.close()


if __name__ == "__main__":
    main()
