use std::fs::File;
use serde::Serialize;
use crate::simulation::SimulationConfig;

#[derive(Serialize)]
struct OutputMetadata {
    sample_rate: u32,
    rays_cast: u32,
    rays_received: u32,
    room_name: String
}

#[derive(Serialize)]
struct OutputHits {
    delays_seconds: Vec<f32>,
    pressures: Vec<f32>,
}

#[derive(Serialize)]
struct IrOutput {
    metadata: OutputMetadata,
    hits: OutputHits
}

pub fn export_results(delays: Vec<f32>, pressure: Vec<f32>, config: &SimulationConfig) {

    let final_data = IrOutput {
        metadata: OutputMetadata {
            sample_rate: 44100,
            rays_cast: config.rays_to_cast,
            rays_received: delays.len() as u32,
            room_name: String::from("MVP_Test_Room_1")
        },
        hits: OutputHits {
            delays_seconds: delays,
            pressures: pressure
        },
    };
    let file = File::create("ir_output.json").expect("Unable to create file");
    serde_json::to_writer_pretty(file, &final_data).expect("Unable to write JSON");
    println!("Simulation done! Wrote {} hits to ir_output.json", final_data.metadata.rays_received);
}