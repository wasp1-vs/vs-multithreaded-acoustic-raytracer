import numpy as np
from convolution import convolve


def test_dirac_convolution():
    sample_rate = 44100

    # 1. Create a Fake IR (exactly 2 hits)
    test_ir_data = {
        "metadata": {"sample_rate": sample_rate},
        "hits": {
            "delays_seconds": [0.5, 1.0],  # Hit at 0.5s and 1.0s
            "pressures": [0.8, 0.4]  # Energy levels
        }
    }

    # 2. Create a perfect Dirac Impulse (One sample of 1.0, the rest 0.0)
    dry_audio = np.zeros(sample_rate * 2)
    dry_audio[0] = 1.0

    # 3. Run the convolution
    wet_audio = convolve(dry_audio, sample_rate, test_ir_data)

    # 4. Prove the math: Check the exact samples
    sample_1 = int(0.5 * sample_rate)
    sample_2 = int(1.0 * sample_rate)

    print(f"Sample at 0.5s: Expected 0.8, Got {wet_audio[sample_1]:.4f}")
    print(f"Sample at 1.0s: Expected 0.4, Got {wet_audio[sample_2]:.4f}")


if __name__ == "__main__":
    test_dirac_convolution()