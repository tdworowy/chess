"""Start TensorBoard for the AI training runs."""

from pathlib import Path
import subprocess
import sys


AI_ROOT = Path(__file__).resolve().parents[1]


def launch_tensorboard(logdir: str | Path | None = None) -> None:
    """Launch TensorBoard using the training log directory by default."""
    resolved_logdir = Path(logdir) if logdir is not None else AI_ROOT / "runs"
    resolved_logdir = resolved_logdir.resolve()
    if not resolved_logdir.exists():
        print(f"Log directory {resolved_logdir} does not exist. Start training first.")
        return

    print(f"Launching TensorBoard with logdir={resolved_logdir}...")
    print("TensorBoard will be available at http://localhost:6006")

    try:
        subprocess.Popen(
            [sys.executable, "-m", "tensorboard.main", "--logdir", str(resolved_logdir)]
        )
    except Exception as error:
        print(f"Failed to launch TensorBoard: {error}")


if __name__ == "__main__":
    launch_tensorboard()
