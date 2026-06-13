# json wird benötigt, um die ir_output.json als Python-Dictionary einzulesen.
import json

# subprocess wird benötigt, um ein externes Programm, hier die Rust-Binary, zu starten.
import subprocess

# Path wird benötigt, um Dateipfade sauber zu prüfen und zu verarbeiten.
from pathlib import Path


# Diese Funktion ist deine Schnittstelle zwischen Python und Rust.
# Sie startet die Rust-Binary, wartet auf das Ende und lädt danach die ir_output.json.
def run_rust_and_load_json(
    rust_binary_path: str,
    json_path: str
) -> dict:

    # Der übergebene Binary-Pfad wird in ein Path-Objekt umgewandelt.
    rust_binary = Path(rust_binary_path).resolve()

    # Der übergebene JSON-Pfad wird ebenfalls in ein Path-Objekt umgewandelt.
    json_file = Path(json_path)

    # Falls die Rust-Binary nicht existiert, wird ein klarer Fehler geworfen.
    if not rust_binary.exists():
        raise FileNotFoundError(f"Rust-Binary nicht gefunden: {rust_binary}")

    # Falls der Pfad existiert, aber keine Datei ist, wird ebenfalls abgebrochen.
    if not rust_binary.is_file():
        raise FileNotFoundError(f"Pfad ist keine Datei: {rust_binary}")

    # Rust schreibt aktuell fest in eine Datei namens ir_output.json.
    # Deshalb prüfen wir, ob der erwartete Dateiname stimmt.
    if json_file.name != "ir_output.json":
        raise ValueError(
            "Rust exportiert aktuell fest nach 'ir_output.json'. "
            f"Übergeben wurde aber: {json_file.name}"
        )

    # Wir suchen den rust_service-Ordner.
    # Dort liegt input_config.json, die Rust standardmäßig nutzt.
    rust_service_dir = rust_binary.parent

    # Wir gehen alle Elternordner der Binary hoch.
    # Sobald wir input_config.json finden, wissen wir: Das ist der richtige Arbeitsordner.
    for parent in rust_binary.parents:
        if (parent / "input_config.json").exists():
            rust_service_dir = parent
            break

    # Das ist die Datei, die Rust nach erfolgreicher Ausführung erzeugen soll.
    output_file = rust_service_dir / "ir_output.json"

    # Falls eine alte ir_output.json existiert, löschen wir sie vorher.
    # So laden wir später nicht versehentlich alte Daten.
    if output_file.exists():
        output_file.unlink()

    # Hier wird die Rust-Binary gestartet.
    # cwd sorgt dafür, dass Rust im rust_service-Ordner läuft.
    result = subprocess.run(
        [str(rust_binary)],
        cwd=rust_service_dir,
        capture_output=True,
        text=True
    )

    # Wenn Rust mit einem Fehler endet, wird der Returncode geprüft.
    # stdout und stderr werden mit ausgegeben, damit man den Fehler debuggen kann.
    if result.returncode != 0:
        raise RuntimeError(
            f"Rust-Binary ist fehlgeschlagen.\n"
            f"Returncode: {result.returncode}\n\n"
            f"STDOUT:\n{result.stdout}\n\n"
            f"STDERR:\n{result.stderr}"
        )

    # Nach dem Rust-Lauf prüfen wir, ob die JSON-Datei wirklich erzeugt wurde.
    if not output_file.exists():
        raise FileNotFoundError(
            f"Rust wurde ausgeführt, aber ir_output.json wurde nicht gefunden: {output_file}"
        )

    # Die JSON-Datei wird geöffnet und in ein Python-Dictionary umgewandelt.
    try:
        with output_file.open("r", encoding="utf-8") as file:
            ir_data = json.load(file)

    # Falls die JSON-Datei kaputt oder ungültig ist, kommt ein verständlicher Fehler.
    except json.JSONDecodeError as error:
        raise ValueError(f"ir_output.json ist keine gültige JSON-Datei: {output_file}") from error

    # Die geladenen IR-Daten werden an main.py bzw. Cems DSP-Core zurückgegeben.
    return ir_data