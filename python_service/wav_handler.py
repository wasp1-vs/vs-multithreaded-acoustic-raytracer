import numpy as np
import soundfile as sf

def read_wav(wav_path: str) -> tuple:
    """
    # Reads a WAV file and returns the audio data as a NumPy array 
    # along with the corresponding sample rate.
    """
    # Read the WAV file using soundfile
    audio_data, sample_rate = sf.read(wav_path)
    
    return audio_data, sample_rate

def write_wav(wav_path: str, audio_data, sample_rate: int) -> None:
    """
    # Normalizes the audio data to prevent clipping and exports it to a WAV file.
    """
    # Find the maximum absolute amplitude in the audio array to check for clipping
    max_amplitude = np.max(np.abs(audio_data))

    # Normalize the audio data if the amplitude exceeds the standard range [-1.0, 1.0]
    if max_amplitude > 1.0:
        audio_data = audio_data / max_amplitude

    # Export the final array to a WAV file without quality loss
    sf.write(wav_path, audio_data, sample_rate)