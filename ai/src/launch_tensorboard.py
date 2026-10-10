import os
import subprocess
import sys


def launch_tensorboard(logdir: str = "./runs/") -> None:
    """
    Launch TensorBoard to visualize training progress.
    """
    if not os.path.exists(logdir):
        print(f"Log directory {logdir} does not exist. Start training first.")
        return

    print(f"Launching TensorBoard with logdir={logdir}...")
    print("TensorBoard will be available at http://localhost:6006")

    try:
        # Launch tensorboard as a separate process
        subprocess.Popen([sys.executable, "-m", "tensorboard.main", "--logdir", logdir])
    except Exception as e:
        print(f"Failed to launch TensorBoard: {e}")


if __name__ == "__main__":
    launch_tensorboard()
