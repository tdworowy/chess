"""Create a static plot of reward, episode length, and evaluation win rate."""

import csv
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


AI_ROOT = Path(__file__).resolve().parents[1]


def _latest_run_rows(path: Path) -> list[dict[str, str]]:
    if not path.exists():
        return []
    with path.open(newline="", encoding="utf-8") as report:
        rows = list(csv.DictReader(report))
    if not rows:
        return []
    latest_run = rows[-1].get("run_id")
    return [row for row in rows if row.get("run_id") == latest_run]


def plot_results(log_folder: str | Path = AI_ROOT / "logs", title: str = "Learning Curve") -> None:
    """Plot evaluation reward, episode length, and win rate for the latest run."""
    log_folder = Path(log_folder)
    results_path = log_folder / "evaluations.npz"
    if not results_path.exists():
        print(f"No evaluation results found at {results_path}")
        return

    results = np.load(results_path)
    timesteps = results["timesteps"]
    results_mean = np.mean(results["results"], axis=1)
    results_std = np.std(results["results"], axis=1)
    ep_lengths_mean = np.mean(results["ep_lengths"], axis=1)

    evaluation_rows = _latest_run_rows(log_folder / "evaluation_history.csv")
    fig, (ax1, ax2, ax3) = plt.subplots(3, 1, figsize=(11, 13), sharex=True)

    ax1.plot(timesteps, results_mean, label="Mean reward")
    ax1.fill_between(
        timesteps,
        results_mean - results_std,
        results_mean + results_std,
        alpha=0.2,
        label="Reward standard deviation",
    )
    ax1.set_ylabel("Reward")
    ax1.set_title(title)
    ax1.grid(True)
    ax1.legend()

    ax2.plot(timesteps, ep_lengths_mean, color="orange", label="Mean episode length")
    ax2.set_ylabel("Agent turns")
    ax2.grid(True)
    ax2.legend()

    if evaluation_rows:
        eval_timesteps = [int(row["timesteps"]) for row in evaluation_rows]
        win_rates = [float(row["win_rate"]) for row in evaluation_rows]
        rolling_rates = [float(row["rolling_win_rate_50"]) for row in evaluation_rows]
        ax3.plot(eval_timesteps, win_rates, marker="o", alpha=0.5, label="This evaluation")
        ax3.plot(eval_timesteps, rolling_rates, label="Rolling win rate (last 50 games)")
        ax3.set_ylim(0, 1)
        ax3.set_ylabel("Win rate")
        ax3.legend()
    else:
        ax3.text(
            0.5,
            0.5,
            "No evaluation outcome report yet. Run the updated trainer first.",
            ha="center",
            va="center",
            transform=ax3.transAxes,
        )
        ax3.set_ylabel("Win rate")

    ax3.set_xlabel("Timesteps")
    for axis in (ax1, ax2, ax3):
        axis.grid(True)

    fig.tight_layout()
    output_path = log_folder / "learning_curve.png"
    fig.savefig(output_path)
    print(f"Plot saved to {output_path}")
    plt.show()


if __name__ == "__main__":
    plot_results()
