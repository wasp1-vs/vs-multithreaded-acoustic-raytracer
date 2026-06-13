use std::f32::consts::PI;
use glam::Vec2;
use rand::RngExt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;
use crate::geometry;
use crate::geometry::{Ray, Wall};

#[derive(Deserialize)]
pub struct SimulationConfig {
    pub max_bounces: u32,
    pub min_pressure: f32,
    pub mic_radius: f32,
    pub mic_position: Vec2,
    pub rays_to_cast: u32,
    pub speaker_position: Vec2,
}

fn generate_initial_ray(config: &SimulationConfig) -> Ray {
    let mut rng = rand::rng();  // Omnidirectional, random rays
    let random_angle: f32 = rng.random_range(0.0..(2.0 * PI));
    let direction = Vec2::new(random_angle.cos(), random_angle.sin());
    Ray {
        origin: config.speaker_position,
        direction,
    }
}

fn simulate_single_ray(
    initial_ray:Ray,
    walls: &Vec<Wall>,
    config: &SimulationConfig,
    ray_hits: &mut Vec<(f32,f32)>
) {
    ray_hits.clear();
    let mut current_ray = initial_ray;
    let mut total_distance = 0.0;
    let mut current_pressure = 1.0;


    for _bounce in 0..config.max_bounces {
        if let Some((bounced_ray, absorption)) = cast_ray(&current_ray, walls) {


            let start_point = current_ray.origin;
            let end_point = bounced_ray.origin;
            if geometry::check_mic_intersection(start_point, end_point, config.mic_position, config.mic_radius) {
                let distance_to_mic = start_point.distance(config.mic_position);
                let total_distance_at_hit = total_distance + distance_to_mic;
                //println!("total dist: {}",total_distance_at_hit);
                if total_distance_at_hit >0.001 { //Spawn epsilon
                    ray_hits.push((total_distance_at_hit / 343.0, current_pressure));
                }
                 // meter / sec
            }
            total_distance += start_point.distance(end_point);
            current_pressure *= 1.0 - absorption;

            if current_pressure < config.min_pressure {
                break;
            }



            current_ray = bounced_ray;

        } else { break; }
    }
}

// Single thread logic
pub fn run_simulation_single(config: &SimulationConfig, walls: &Vec<Wall>) -> (Vec<f32>, Vec<f32>) {
    let mut delays_singular = Vec::with_capacity(100_000);
    let mut pressures_singular = Vec::with_capacity(100_000);
    let mut temp_ray_buffer = Vec::with_capacity(100);
    for _ in 0..config.rays_to_cast {
        let current_ray = generate_initial_ray(config);
        simulate_single_ray(current_ray, walls, config, &mut temp_ray_buffer);
        for (delay,pressure) in &temp_ray_buffer {
            delays_singular.push(*delay);
            pressures_singular.push(*pressure);
        }
    }
    (delays_singular, pressures_singular)
}

pub fn run_simulation_parallel(config: &SimulationConfig, walls: &Vec<Wall>) -> (Vec<f32>, Vec<f32>) {
    println!("starting execution");
    let(delays_par, pressures_par, _) = (0..config.rays_to_cast)
        .into_par_iter()
        .fold( || {
            let local_delays = Vec::with_capacity(100_000);
            let local_pressures = Vec::with_capacity(100_000);
            let temp_ray_buffer = Vec::with_capacity(100);

            (local_delays,local_pressures,temp_ray_buffer)
        },
            |mut thread_buckets, _| {
                let fresh_ray = generate_initial_ray(config);
                simulate_single_ray(fresh_ray,walls,config,&mut thread_buckets.2); // passing temp buffer to not allocate new memory
                for(delay, pressure) in &thread_buckets.2 {
                    thread_buckets.0.push(*delay);
                    thread_buckets.1.push(*pressure);
                }
                thread_buckets
            }
        ).reduce( // Merge threadbuckets into one final list
        || (Vec::new(),Vec::new(),Vec::new()),
        |mut a, mut b| {
            a.0.append(&mut b.0);
            a.1.append(&mut b.1);
            a
        }
    );
    (delays_par, pressures_par)
}

fn cast_ray(ray: &Ray, walls: &Vec<Wall>) -> Option<(Ray, f32)> {
    let mut closest_bounced_ray: Option<(Ray, f32)> = None;
    // To guarantee that the first wall hit will be shorter
    let mut shortest_distance = f32::MAX;
    for wall in walls {
        if let Some(bounced_ray) = geometry::check_intersection(ray, wall) {
            let distance = ray.origin.distance(bounced_ray.origin);
            if distance < shortest_distance {
                shortest_distance = distance;
                closest_bounced_ray = Some((bounced_ray, wall.absorption));
            }
        }
    }

    closest_bounced_ray
}

pub fn simulate_single_ray_visualisation(
    initial_ray:Ray,
    walls: &Vec<Wall>,
    config: &SimulationConfig,
    path_buffer: &mut Vec<Vec2>
) {
    path_buffer.clear();
    path_buffer.push(initial_ray.origin);
    let mut current_ray = initial_ray;
    let mut current_pressure = 1.0;


    for _bounce in 0..config.max_bounces {
        if let Some((bounced_ray, absorption)) = cast_ray(&current_ray, walls) {

            path_buffer.push(bounced_ray.origin);
            current_pressure *= 1.0 - absorption;

            if current_pressure < config.min_pressure {
                break;
            }
            current_ray = bounced_ray;

        } else { break; }
    }
}
pub fn run_simulation_visualizer(
    rays_to_trace: u32,
    config: &SimulationConfig,
    walls: &Vec<Wall>
) -> Vec<Vec<Vec2>> {
    let mut all_paths = Vec::with_capacity(rays_to_trace as usize);
    let mut temp_path = Vec::with_capacity(config.max_bounces as usize + 1);
    for _ in 0..rays_to_trace {
        let current_ray = generate_initial_ray(config);
        simulate_single_ray_visualisation(current_ray, walls, config, &mut temp_path);

        all_paths.push(temp_path.clone());

    }
    all_paths
}
