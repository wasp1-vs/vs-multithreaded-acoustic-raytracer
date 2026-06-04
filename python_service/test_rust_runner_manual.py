# Importiert die Rust-Python-Schnittstelle
from rust_runner import run_rust_and_load_json


def main():
    print("Testing Rust runner...")

    # Pfad zur kompilierten Rust-Binary
    rust_binary = "../rust_service/target/release/rust_service.exe"

    # Datei, die Rust erzeugt
    json_output = "ir_output.json"

    # Rust starten und JSON laden
    ir_data = run_rust_and_load_json(
        rust_binary,
        json_output
    )

    # Prüfen, ob ein Dictionary zurückkommt
    print(type(ir_data))

    # Prüfen, ob metadata und hits vorhanden sind
    print(ir_data.keys())

    # Metadaten anzeigen
    print(ir_data["metadata"])

    # Prüfen, ob delays_seconds und pressures vorhanden sind
    print(ir_data["hits"].keys())

    # Prüfen, wie viele Delays geladen wurden
    print(len(ir_data["hits"]["delays_seconds"]))

    # Prüfen, wie viele Pressure-Werte geladen wurden
    print(len(ir_data["hits"]["pressures"]))


if __name__ == "__main__":
    main()