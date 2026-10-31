# Checkers AI training

The MaskablePPO trainer reports progress in the terminal, TensorBoard, and CSV files. It evaluates the policy against the API's easy opponent, so the win-rate checks describe performance against that opponent.

## Start training

Start the Checkers API on `http://localhost:8080`, then run from this directory:

```powershell
uv sync
uv run python src/train.py
```

Set `CHECKERS_API_URL` to use a different API address. Set `CHECKERS_TRAINING_TIMESTEPS` to change the default 200,000 training steps.

The terminal shows a timestep progress bar, training episode outcomes, and an evaluation summary every 5,000 steps. Evaluation plays 10 games. Games that reach 200 agent turns are counted as draws, so a repeating game cannot stall training indefinitely. API requests time out after 30 seconds.

## View charts

In another terminal, run from `ai/`:

```powershell
uv run python src/launch_tensorboard.py
```

Open [http://localhost:6006](http://localhost:6006). The `eval` charts show win rate, score rate, shaped reward, and episode length. The `train` charts show recent episode outcomes and invalid actions.

Training writes these files under `ai/` regardless of the current working directory:

- `runs/`: TensorBoard event files.
- `logs/evaluations.npz`: evaluation rewards and episode lengths.
- `logs/evaluation_history.csv`: wins, losses, draws, rolling rates, reward, and game length at each evaluation.
- `logs/training_episodes.csv`: result and reward for each completed training game.
- `logs/best_model.zip`: model with the best evaluation score rate (wins plus half a point for draws).
- `checkers_ppo.zip`: final model from the run.

The trainer prints a warning if action masking produces an invalid move, a game ends without outcome information, no games finish within 5,000 steps, win rate stays below 25% across at least 30 recent evaluation games, or score rate remains below 75% without a new high for five checks. These thresholds are debugging signals, not a guarantee of strength; review both the outcome and reward charts when deciding whether a run is useful.

To make a static chart, run `uv run python src/visualize_training.py` from `ai/`. It saves `logs/learning_curve.png` with reward, game length, and win-rate history.
