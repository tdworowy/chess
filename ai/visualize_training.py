import os

import matplotlib.pyplot as plt
import numpy as np


def plot_results(log_folder, title="Learning Curve"):
    """
    plot the results

    :param log_folder: (str) the save location of the results to plot
    :param title: (str) the title of the plot
    """
    results_path = os.path.join(log_folder, "evaluations.npz")
    if not os.path.exists(results_path):
        print(f"No evaluation results found at {results_path}")
        return

    results = np.load(results_path)

    timesteps = results["timesteps"]
    results_mean = np.mean(results["results"], axis=1)
    results_std = np.std(results["results"], axis=1)

    ep_lengths = results["ep_lengths"]
    ep_lengths_mean = np.mean(ep_lengths, axis=1)

    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 10), sharex=True)

    # Reward Plot
    ax1.plot(timesteps, results_mean, label="Mean Reward")
    ax1.fill_between(
        timesteps, results_mean - results_std, results_mean + results_std, alpha=0.2
    )
    ax1.set_ylabel("Reward")
    ax1.set_title(title)
    ax1.grid(True)
    ax1.legend()

    # Episode Length Plot
    ax2.plot(timesteps, ep_lengths_mean, color="orange", label="Mean Episode Length")
    ax2.set_xlabel("Timesteps")
    ax2.set_ylabel("Steps")
    ax2.set_title("Episode Length")
    ax2.grid(True)
    ax2.legend()

    plt.tight_layout()
    plt.savefig(os.path.join(log_folder, "learning_curve.png"))
    print(f"Plot saved to {os.path.join(log_folder, 'learning_curve.png')}")
    plt.show()


if __name__ == "__main__":
    plot_results("./logs/")
