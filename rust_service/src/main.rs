mod geometry;
mod simulation;
mod export;

use clap::Parser;
use geometry::Wall;
use glam::Vec2;
use simulation::SimulationConfig;
use std::fs;
use std::time::Instant;


#[derive(Parser)]
#[command(name = "Acoustic Engine")]
#[command(about = "Multithreaded acoustic raytracer", long_about = None)]
/// usage: cargo run --release -- --config-file (input_config.json)
struct Cli {
    /// Path to the JSON config file
    #[arg(short, long, default_value = "input_config.json")]
    config_file: String,
}
fn main() {
    //
    // let test_wall1 = Wall {
    //     start:Vec2::new(50.0,0.0),
    //     end:Vec2::new(50.0,50.0),
    //     absorption: 1.0
    // };
    // let test_wall2 = Wall {
    //     start:Vec2::new(50.0,50.0),
    //     end:Vec2::new(0.0,50.0),
    //     absorption: 1.0
    // };
    // let test_wall3 = Wall {
    //     start:Vec2::new(0.0,50.0),
    //     end:Vec2::new(0.0,0.0),
    //     absorption: 1.0
    // };
    // let test_wall4 = Wall {
    //     start:Vec2::new(0.0,0.0),
    //     end:Vec2::new(50.0,0.0),
    //     absorption: 1.0
    // };
    // let room: Vec<Wall> = vec![test_wall1, test_wall2, test_wall3, test_wall4];
    // The reflective canyon wall we want to test
    let front_wall = Wall {
        start: Vec2::new(343.0, -1000.0),
        end: Vec2::new(343.0, 1000.0),
        absorption: 0.0
    };

    // The "catcher" wall behind the speaker.
    // Set absorption to 1.0 so the ray dies immediately after hitting it.
    let back_wall = Wall {
        start: Vec2::new(-10.0, -1000.0),
        end: Vec2::new(-10.0, 1000.0),
        absorption: 0.0
    };
    let room: Vec<Wall> = vec![front_wall, back_wall];
    let cli = Cli::parse();
    println!("Reading config from: {}", cli.config_file);
    let config_text = fs::read_to_string(&cli.config_file)
        .expect("Could not read config file");
    let test_config: SimulationConfig = serde_json::from_str(&config_text)
        .expect("Could not parse simulation config (Is the JSON formatted correctly?)");


    // println!("Starting single simulation");
    // let start_single = Instant::now();
    // let(_delays_single, _pressures_single) = simulation::run_simulation_single(&test_config, &room);
    // let duration_singe = start_single.elapsed().as_secs_f32();
    // println!("Simulation run time: {} s", duration_singe);

    println!("Starting parallel simulation");
    let start_parallel = Instant::now();
    let(delays_par, pressures_par) = simulation::run_simulation_parallel(&test_config, &room);
    let duration_parallel = start_parallel.elapsed().as_secs_f32();
    println!("Simulation run time: {} s", duration_parallel);

    // let speedup = duration_singe / duration_parallel;
    // println!("\nSpeedup time: {:.2}x s", speedup);

    export::export_results(delays_par, pressures_par, &test_config);


    let rays_to_visualize =simulation::run_simulation_visualizer(15, &test_config, &room);
    export::export_visualisation_data(rays_to_visualize, &test_config);


}
