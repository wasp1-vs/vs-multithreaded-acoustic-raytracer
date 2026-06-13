import json
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D 
from pathlib import Path

def render_debug_paths_3d(file_path):
    with open(file_path, 'r') as file:
        data = json.load(file)

    rays = data['rays']

    fig = plt.figure(figsize=(10, 10))
    ax = fig.add_subplot(111, projection='3d') # Aktiviert den 3D-Modus
    ax.set_title(f"3D Acoustic Ray Tracing Debugger ({len(rays)} Rays)")

    for i, ray in enumerate(rays):
        # Check if the ray has 3D coordinates (x, y, z) or 2D (x, y)
        if len(ray[0]) == 3:
            xs, ys, zs = zip(*ray)
            ax.plot(xs, ys, zs, linestyle='-', linewidth=1.0, alpha=0.5)
        else:
            xs, ys = zip(*ray)
            # Plot as 2D in 3D space (set Z=0)
            zs = [0] * len(xs)
            ax.plot(xs, ys, zs, linestyle='-', linewidth=1.0, alpha=0.5)

    ax.set_xlabel("X (meters)")
    ax.set_ylabel("Y (meters)")
    ax.set_zlabel("Z (meters)")
    
    print("Launching 3D visualizer...")
    plt.show()

if __name__ == "__main__":
    script_dir = Path(__file__).parent
    target_file = script_dir.parent / "rust_service" / "visualisation_data.json"
    render_debug_paths_3d(target_file)