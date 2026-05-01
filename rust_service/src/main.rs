use glam::Vec2;
use rand::RngExt;
use serde::Serialize;
use std::f32::consts::PI;
use std::fs::File;

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
struct Ray {
    origin: Vec2,     // O
    direction: Vec2   // D
}
struct Wall {
    start: Vec2, // A
    end: Vec2,   // B
    absorption: f32,
}
struct SimulationConfig {
    max_bounces: u32,
    min_pressure: f32,
    mic_radius: f32,
    mic_position: Vec2,
    rays_to_cast: u32,
    speaker_position: Vec2,
}
fn check_intersection(ray: &Ray, wall: &Wall) -> Option<Ray> {
    let from_ray_to_wall_start = wall.start - ray.origin; //Vec2::new(wall.start.x - ray.origin.x, wall.start.y - ray.origin.y); // E
    let wall_vec = wall.end - wall.start;//Vec2::new(wall.end.x - wall.start.x, wall.end.y - wall.start.y); // V

    let det = ray.direction.perp_dot(wall_vec);
    if det.abs() < 1e-6 {
        return None;
    }
    let ray_t = (from_ray_to_wall_start.perp_dot(wall_vec))/det;        // how far the ray travels
    let wall_u = (from_ray_to_wall_start.perp_dot(ray.direction))/det;  // where on the wall it hits

     if ray_t < 0.0 {
        return None;
    } else if wall_u < 0.0 || wall_u > 1.0 {
        return None;
    }
    let hit_point = ray.origin + (ray_t * wall_u);
    let wall_u_normalized = Vec2::new(-wall_vec.y, wall_vec.x).normalize();
    let bounced_ray = ray.direction.reflect(wall_u_normalized);


    Some(Ray {
        origin: hit_point,
        direction: bounced_ray,
    })
}
fn check_mic_intersection(
    start: Vec2,
    end: Vec2,
    mic_center: Vec2,
    mic_radius: f32
) -> bool {
    let segment = end - start;
    let segment_length_sq = segment.length_squared();

    if segment_length_sq == 0.0 { // if ray didnt move at all
        return start.distance(mic_center) <= mic_radius;
    }
    let to_mic = mic_center - start; // Vector pointing from start of the ray to the microphone
    let t = to_mic.dot(segment) / segment_length_sq; // Vector Projection
    let clamped_t = t.clamp(0.0, 1.0); // Only the segment not the inf line
    let closest_point = start + (segment * clamped_t); // Coordinates of the closes point on the segment
    closest_point.distance(mic_center) <= mic_radius // Is the closes point inside the circle?

}

fn cast_ray(ray: &Ray, walls: &Vec<Wall>) -> Option<(Ray, f32)> {
    let mut closest_bounced_ray: Option<(Ray, f32)> = None;
    // To guarantee that the first wall hit will be shorter
    let mut shortest_distance = f32::MAX;
    for wall in walls {
        if let Some(bounced_ray) = check_intersection(ray, wall) {
            let distance = ray.origin.distance(bounced_ray.origin);
            if distance < shortest_distance {
                shortest_distance = distance;
                closest_bounced_ray = Some((bounced_ray, wall.absorption));
            }
        }
    }

    closest_bounced_ray
}

fn generate_initial_ray(config: &SimulationConfig) -> Ray {
    let mut rng = rand::rng();
    let random_angle: f32 = rng.random_range(0.0..(2.0 * PI));
    let direction = Vec2::new(random_angle.cos(), random_angle.sin());
    Ray {
        origin: config.speaker_position,
        direction,
    }
}
fn simulate_single_ray(initial_ray:Ray, walls: &Vec<Wall>, config: &SimulationConfig) -> Vec<(f32, f32)> {
    let mut current_ray = initial_ray;
    let mut total_distance = 0.0;
    let mut current_pressure = 1.0;
    let mut ray_hits: Vec<(f32, f32)> = Vec::new();


    for _bounce in 0..config.max_bounces {
        if let Some((bounced_ray, absorption)) = cast_ray(&current_ray, walls) {

            total_distance += current_ray.origin.distance(bounced_ray.origin);
            current_pressure *= 1.0 - absorption;

            if current_pressure < config.min_pressure {
                break;
            }
            let start_point = current_ray.origin;
            let end_point = bounced_ray.origin;
            if check_mic_intersection(start_point,end_point,config.mic_position,config.mic_radius) {
                let distance_to_mic = start_point.distance(config.mic_position);
                let total_distance_at_hit = total_distance + distance_to_mic;
                let delay_seconds = total_distance_at_hit / 343.0; // meters per second
                ray_hits.push((delay_seconds, current_pressure));
            }

            current_ray = bounced_ray;

        } else { break; }
    }
    ray_hits
}
fn export_results(delays: Vec<f32>, pressure: Vec<f32>,config: &SimulationConfig) {

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

fn run_simulation(config: &SimulationConfig, walls: &Vec<Wall>) {
    let mut delays_array: Vec<f32> = Vec::new();
    let mut pressures_array: Vec<f32> = Vec::new();
    for _ in 0..config.rays_to_cast {
        let current_ray = generate_initial_ray(config);
        let ray_hits = simulate_single_ray(current_ray, walls, config);
        for (delay,pressure) in ray_hits {
            delays_array.push(delay);
            pressures_array.push(pressure);
        }
    }
    export_results(delays_array,pressures_array,config);
}

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
        max_bounces: 32,
        min_pressure: 0.001,
        mic_radius: 1.5,
        mic_position:Vec2::new(5.0,5.0),
        rays_to_cast: 100,
        speaker_position:Vec2::new(2.0,2.0),
    };
    run_simulation(&test_config, &room);
}
