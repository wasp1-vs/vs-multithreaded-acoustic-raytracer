import json
import matplotlib.pyplot as plt
from pathlib import Path

def render_debug_paths(file_path):
    with open(file_path, 'r') as file:
        data = json.load(file)

    speaker = data['speaker']
    mic = data['mic']
    mic_radius = data['mic_radius']
    rays = data['rays']

    fig, ax = plt.subplots(figsize=(10,10))
    ax.set_aspect('equal')
    ax.set_title(f"Acoustic Ray Tracing Debugger ({len(rays)}) Rays")

    mic_circle = plt.Circle((mic[0], mic[1]), mic_radius,color='red', alpha=0.3, label='Mic Radius')
    ax.add_patch(mic_circle)
    ax.plot(mic[0], mic[1], 'bo', markersize=5, label='Mic Center')

    ax.plot(speaker[0], speaker[1], 'r*', markersize=12, label='Speaker')
    room_walls = plt.Rectangle((0,0), 10, 10, fill=False, color='black', linewidth=2, label='Room Walls')
    ax.add_patch(room_walls)

    for i, ray in enumerate(rays):
        xs, ys = zip(*ray)

        label = "Ray Path" if i == 0 else None
        ax.plot(xs, ys, linestyle='-',linewidth= 1.5, alpha=0.5, label=label)

    ax.grid(True, linestyle='-', alpha=0.6)
    ax.set_xlabel("X axis (Meters)")
    ax.set_ylabel("Y axis (Meters)")
    ax.legend(loc='upper right')
    print("Launcher visualizer")
    plt.show()

if __name__ == "__main__":
    script_dir = Path(__file__).parent
    target_file = script_dir.parent / "rust_service" / "visualisation_data.json"
    render_debug_paths(target_file)