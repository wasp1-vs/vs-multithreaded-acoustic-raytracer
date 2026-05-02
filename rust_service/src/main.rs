mod geometry;
mod simulation;
mod export;

use glam::Vec2;
use std::time::Instant;
use geometry::{Ray, Wall};
use simulation::SimulationConfig;

fn main() {
    let _test_ray = Ray {
        origin: Vec2::new(2.0,2.0),
        direction: Vec2::new(1.0,0.5).normalize(),
    };
    let test_wall1 = Wall {
        start:Vec2::new(10.0,0.0),
        end:Vec2::new(10.0,10.0),
        absorption: 0.1
    };
    let test_wall2 = Wall {
        start:Vec2::new(10.0,10.0),
        end:Vec2::new(0.0,10.0),
        absorption: 0.1
    };
    let test_wall3 = Wall {
        start:Vec2::new(0.0,10.0),
        end:Vec2::new(0.0,0.0),
        absorption: 0.1
    };
    let test_wall4 = Wall {
        start:Vec2::new(0.0,0.0),
        end:Vec2::new(10.0,0.0),
        absorption: 0.1
    };
    let room: Vec<Wall> = vec![test_wall1, test_wall2, test_wall3, test_wall4];
    let test_config = SimulationConfig {
        max_bounces: 320,
        min_pressure: 0.001,
        mic_radius: 1.5,
        mic_position:Vec2::new(5.0,5.0),
        rays_to_cast: 100000,
        speaker_position:Vec2::new(2.0,2.0),
    };
    println!("Starting single simulation");
    let start_single = Instant::now();
    let(delays_single, pressures_single) = simulation::run_simulation_single(&test_config, &room);
    let duration_singe = start_single.elapsed().as_secs_f32();
    println!("Simulation run time: {} s", duration_singe);

    println!("Starting parallel simulation");
    let start_parallel = Instant::now();
    let(delays_par, pressures_par) = simulation::run_simulation_parallel(&test_config, &room);
    let duration_parallel = start_parallel.elapsed().as_secs_f32();
    println!("Simulation run time: {} s", duration_parallel);

    let speedup = duration_singe / duration_parallel;
    println!("\nSpeedup time: {:.2}x s", speedup);

    export::export_results(delays_par, pressures_par, &test_config);
}
