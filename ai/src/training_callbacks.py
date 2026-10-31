"""Callbacks that make training quality visible in the console and in reports."""

import csv
from collections import Counter, deque
from pathlib import Path
from typing import Any

import numpy as np
from sb3_contrib.common.maskable.callbacks import MaskableEvalCallback
from stable_baselines3.common.callbacks import BaseCallback


def _append_csv(path: Path, row: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    file_exists = path.exists() and path.stat().st_size > 0
    with path.open("a", newline="", encoding="utf-8") as report:
        writer = csv.DictWriter(report, fieldnames=list(row))
        if not file_exists:
            writer.writeheader()
        writer.writerow(row)


def _outcome(info: dict[str, Any], agent_color: str) -> str:
    winner = info.get("winner")
    if winner == agent_color:
        return "win"
    if winner:
        return "loss"
    if info.get("TimeLimit.truncated"):
        return "draw"
    return "unknown"


class TrainingEpisodeCallback(BaseCallback):
    """Report recent training wins, losses, draws, and invalid actions."""

    def __init__(
        self,
        report_path: Path,
        run_id: str,
        agent_color: str,
        report_every_episodes: int = 25,
    ) -> None:
        super().__init__()
        self.report_path = report_path
        self.run_id = run_id
        self.agent_color = agent_color
        self.report_every_episodes = report_every_episodes
        self.episode_count = 0
        self.outcomes: deque[str] = deque(maxlen=100)
        self.invalid_actions: deque[bool] = deque(maxlen=100)
        self._warned_about_missing_episodes = False

    def _on_step(self) -> bool:
        dones = self.locals.get("dones", [])
        infos = self.locals.get("infos", [])

        for done, info in zip(dones, infos):
            if not done:
                continue

            episode_info = info.get("episode", {})
            outcome = _outcome(info, self.agent_color)
            invalid_action = bool(info.get("invalid_action", False))
            self.episode_count += 1
            self.outcomes.append(outcome)
            self.invalid_actions.append(invalid_action)

            _append_csv(
                self.report_path,
                {
                    "run_id": self.run_id,
                    "timesteps": self.num_timesteps,
                    "episode": self.episode_count,
                    "outcome": outcome,
                    "reward": episode_info.get("r", ""),
                    "length": episode_info.get("l", ""),
                    "invalid_action": invalid_action,
                },
            )

            counts = Counter(self.outcomes)
            recent_count = len(self.outcomes)
            self.logger.record("train/win_rate_100", counts["win"] / recent_count)
            self.logger.record("train/loss_rate_100", counts["loss"] / recent_count)
            self.logger.record("train/draw_rate_100", counts["draw"] / recent_count)
            self.logger.record(
                "train/invalid_action_rate_100",
                sum(self.invalid_actions) / len(self.invalid_actions),
            )
            self.logger.record("train/episode_outcome", {"win": 1, "loss": -1}.get(outcome, 0))

            if invalid_action:
                print(
                    f"WARNING at {self.num_timesteps:,} steps: the policy selected an invalid action. "
                    "Action masking should prevent this; check the observation/action encoding."
                )
            if outcome == "unknown":
                print(
                    f"WARNING at {self.num_timesteps:,} steps: an episode ended without a winner "
                    "or a time-limit draw. Check the environment's terminal info."
                )

            if self.episode_count % self.report_every_episodes == 0:
                print(
                    f"Train | step {self.num_timesteps:,} | episode {self.episode_count:,} | "
                    f"last {recent_count}: {counts['win']}W/{counts['loss']}L/"
                    f"{counts['draw']}D/{counts['unknown']}? | "
                    f"invalid actions: {sum(self.invalid_actions)}"
                )

        if (
            self.num_timesteps >= 5_000
            and self.episode_count == 0
            and not self._warned_about_missing_episodes
        ):
            print(
                "WARNING: no training games have completed after 5,000 steps. "
                "Check that the Checkers API is responding and that games can reach a terminal state."
            )
            self._warned_about_missing_episodes = True

        return True

    def _on_training_end(self) -> None:
        if not self.outcomes:
            return
        counts = Counter(self.outcomes)
        print(
            f"Training episodes (last {len(self.outcomes)}): {counts['win']}W/"
            f"{counts['loss']}L/{counts['draw']}D/{counts['unknown']}?"
        )


class EvaluationDiagnosticsCallback(MaskableEvalCallback):
    """Evaluate game outcomes, log win-rate charts, and flag weak/stalled runs."""

    def __init__(
        self,
        *args: Any,
        report_path: Path,
        run_id: str,
        agent_color: str,
        **kwargs: Any,
    ) -> None:
        best_model_save_path = kwargs.pop("best_model_save_path", None)
        self.quality_best_model_path = (
            Path(best_model_save_path) / "best_model"
            if best_model_save_path is not None
            else None
        )
        # The built-in callback selects by shaped reward. Save our best model
        # using actual game score instead.
        kwargs["best_model_save_path"] = None
        super().__init__(*args, **kwargs)
        self.report_path = report_path
        self.run_id = run_id
        self.agent_color = agent_color
        self._current_outcomes: Counter[str] = Counter()
        self._current_outcome_sequence: list[str] = []
        self._recent_outcomes: deque[str] = deque(maxlen=50)
        self._recent_scores: deque[float] = deque(maxlen=5)
        self._best_score_rate = -1.0
        self._checks_without_improvement = 0
        self._weak_play_warning_issued = False
        self._stagnation_warning_issued = False

    def _log_success_callback(self, locals_: dict[str, Any], globals_: dict[str, Any]) -> None:
        super()._log_success_callback(locals_, globals_)
        if not locals_.get("done"):
            return

        info = locals_.get("info", {})
        outcome = _outcome(info, self.agent_color)
        self._current_outcomes[outcome] += 1
        self._current_outcome_sequence.append(outcome)

    def _on_step(self) -> bool:
        evaluation_due = self.eval_freq > 0 and self.n_calls % self.eval_freq == 0
        previous_evaluation_count = len(self.evaluations_timesteps)
        if evaluation_due:
            self._current_outcomes = Counter()
            self._current_outcome_sequence = []
            print(
                f"Starting evaluation at step {self.num_timesteps:,} "
                f"({self.n_eval_episodes} games)..."
            )

        continue_training = super()._on_step()
        if evaluation_due and len(self.evaluations_timesteps) > previous_evaluation_count:
            self._report_evaluation()
        return continue_training

    def _report_evaluation(self) -> None:
        wins = self._current_outcomes["win"]
        losses = self._current_outcomes["loss"]
        draws = self._current_outcomes["draw"]
        unknown = self._current_outcomes["unknown"]
        games = wins + losses + draws + unknown
        win_rate = wins / games if games else 0.0
        score_rate = (wins + 0.5 * draws) / games if games else 0.0
        mean_reward = float(np.mean(self.evaluations_results[-1]))
        mean_length = float(np.mean(self.evaluations_length[-1]))

        self._recent_outcomes.extend(self._current_outcome_sequence)
        self._recent_scores.append(score_rate)
        recent_counts = Counter(self._recent_outcomes)
        recent_games = len(self._recent_outcomes)
        recent_win_rate = recent_counts["win"] / recent_games if recent_games else 0.0
        recent_score_rate = (
            (recent_counts["win"] + 0.5 * recent_counts["draw"]) / recent_games
            if recent_games
            else 0.0
        )
        rolling_score_rate = float(np.mean(self._recent_scores))

        self.logger.record("eval/win_rate", win_rate)
        self.logger.record("eval/score_rate", score_rate)
        self.logger.record("eval/win_rate_recent_50", recent_win_rate)
        self.logger.record("eval/score_rate_recent_50", recent_score_rate)
        self.logger.record("eval/score_rate_rolling_5", rolling_score_rate)
        self.logger.record("eval/wins", wins)
        self.logger.record("eval/losses", losses)
        self.logger.record("eval/draws", draws)
        self.logger.record("eval/unknown_outcomes", unknown)

        _append_csv(
            self.report_path,
            {
                "run_id": self.run_id,
                "timesteps": self.num_timesteps,
                "wins": wins,
                "losses": losses,
                "draws": draws,
                "unknown": unknown,
                "win_rate": win_rate,
                "score_rate": score_rate,
                "rolling_win_rate_50": recent_win_rate,
                "rolling_score_rate_50": recent_score_rate,
                "mean_reward": mean_reward,
                "mean_episode_length": mean_length,
            },
        )

        print(
            f"Eval | step {self.num_timesteps:,} | {wins}W/{losses}L/{draws}D/{unknown}? "
            f"({win_rate:.0%} wins, {score_rate:.0%} points) | "
            f"reward {mean_reward:.2f} | mean turns {mean_length:.1f} | "
            f"rolling win rate {recent_win_rate:.0%}"
        )

        if unknown:
            print(
                f"WARNING: {unknown} evaluation game(s) ended without a winner or time-limit draw. "
                "Check the environment's terminal info."
            )

        if recent_games >= 30 and recent_win_rate < 0.25 and not self._weak_play_warning_issued:
            print(
                f"WARNING: evaluation win rate is only {recent_win_rate:.0%} over the last "
                f"{recent_games} games. The policy may be learning poorly; compare the reward and "
                "win-rate charts and inspect the API/environment behavior."
            )
            self._weak_play_warning_issued = True

        previous_best_score_rate = self._best_score_rate
        is_new_best = score_rate > previous_best_score_rate
        if is_new_best:
            self._best_score_rate = score_rate
            if self.quality_best_model_path is not None:
                self.quality_best_model_path.parent.mkdir(parents=True, exist_ok=True)
                self.model.save(str(self.quality_best_model_path))
                saved_model = self.quality_best_model_path.with_suffix(".zip")
                print(f"New best evaluation score; saved {saved_model}")

        if is_new_best:
            self._checks_without_improvement = 0
        else:
            self._checks_without_improvement += 1

        if (
            self._best_score_rate < 0.75
            and self._checks_without_improvement >= 5
            and not self._stagnation_warning_issued
        ):
            print(
                "WARNING: evaluation score is below 75% and has not set a new high for 5 checks. "
                "Inspect the win-rate, reward, and episode-length charts for a plateau or environment issue."
            )
            self._stagnation_warning_issued = True

        # MaskableEvalCallback has already dumped its reward metrics. Dump these
        # outcome metrics at the same timestep so TensorBoard shows them promptly.
        self.logger.dump(self.num_timesteps)
